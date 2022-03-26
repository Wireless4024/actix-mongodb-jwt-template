use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;
#[cfg(feature = "basic-auth")]
use std::sync::Arc;

use actix_web::{dev, Error, FromRequest, HttpRequest, web};
use actix_web::error::ErrorUnauthorized;
#[cfg(feature = "basic-auth")]
use actix_web::web::Data;
use anyhow::Result;
use chrono::Duration;
use futures::future::ready;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};

use crate::manager::DatabaseWrapper;
use crate::schema::Jwt;
use crate::util::env::env;
use crate::util::time::{timestamp_u64, TimestampExt};

use super::login_by_username;

const JWT_EXPIRE_HOUR: u64 = 24;

#[cfg(feature = "static-jwt-secret")]
static SECRET: &'static str = include_str!("../../jwt_secret");

/// get expire timestamp for jwt
fn jwt_expire_time() -> u64 {
	Duration::hours(
		env("AUTH_JWT_EXPIRE_HOUR")
			// load as u64 because expire time -1 hour doesn't make sense
			.and_then(|it| u64::from_str(it.as_str()).ok())
			// minimum token time 1 hour
			.map(|hours| if hours < 1 { 1 } else { 0 })
			.unwrap_or(JWT_EXPIRE_HOUR)
			as i64)
		// cast to u64 due we don't need to use timestamp < 0 it's older than 1970
		.timestamp_from_now() as u64
}

/// get default header for jwt
fn default_jwt_header() -> Header {
	let mut header = Header::default();
	// sha512 provide better security and faster on large data
	header.alg = Algorithm::HS512;
	header
}

lazy_static::lazy_static! {
	static ref JWT_KEY: (EncodingKey, DecodingKey) = {
		#[cfg(not(feature = "static-jwt-secret"))]
		{
			use crate::util::env::raw_env;
			let data = raw_env("AUTH_JWT_SECRET").unwrap_or_default();
			if data.is_empty() {
				panic!("please set `JWT_SECRET` in environment variable")
			}
			(EncodingKey::from_secret(&data), DecodingKey::from_secret(&data))
		}
		#[cfg(feature = "static-jwt-secret")]
		{
			let data = SECRET.as_bytes();
			if data.is_empty() {
				panic!("please add secret to `jwt_secret` file")
			}
			(EncodingKey::from_secret(&data), DecodingKey::from_secret(&data))
		}
	};
}

/// this function use to create JWT token from args (this may call from different authenticate method)
pub async fn create_token(sub: String) -> Result<String> {
	Ok(web::block(move || {
		let exp = jwt_expire_time();
		let claims = Jwt {
			sub,
			exp,
		};
		encode(&default_jwt_header(), &claims, &JWT_KEY.0)
	}).await??)
}

/// login with `username` and `password` and return JWT token
pub async fn login_as_token(db: impl Deref<Target=DatabaseWrapper>, username: &str, password: &str) -> Result<String> {
	let user = login_by_username(db, username, password).await?;
	Ok(create_token(user.id_ref().to_string()).await?)
}

/// ## Enabling
/// add "basic-auth" to default feature in cargo.toml
///
/// ## Request
/// ```shell
/// # curl schema
/// curl username:password@$HOST
/// ```
/// ```http
/// GET /endpoint
/// Authorization: Basic <base64 encoded username:password>
/// ```
/// ## Response
/// don't have response but will have same behavior as request via jwt token
#[cfg(feature = "basic-auth")]
async fn async_basic_auth(db: Arc<DatabaseWrapper>, data: Option<String>) -> JWTResult {
	if let Some(data) = data {
		let mut split = data.splitn(3, ":");
		if let (Some(username), Some(password)) = (split.next(), split.next()) {
			if let Ok(user) = login_by_username(db.as_ref(), username, password).await {
				return Ok(Jwt {
					sub: user.id_ref().to_string(),
					exp: u64::MAX,// it doesn't even generate jwt token, unused
				});
			}
		}
	}

	JWTResult::Err(ErrorUnauthorized("Invalid username or password!"))
}

type JWTResult = Result<Jwt, Error>;

impl FromRequest for Jwt {
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output=JWTResult>>>;

	fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
		let auth = req.headers().get("Authorization");
		match auth {
			Some(auth) => {
				if auth.len() < 8 { return Box::pin(ready(Err(ErrorUnauthorized("Invalid token!")))); }

				let mut split = auth.to_str().unwrap_or("").splitn(2, " ");
				let auth_type = split.next();

				let token = split.next().unwrap_or("").trim();

				if token.is_empty() {
					return Box::pin(ready(Err(ErrorUnauthorized("Invalid token!"))));
				}
				match auth_type {
					Some("Bearer") => {
						match decode::<Jwt>(
							token,
							&JWT_KEY.1,
							&Validation::new(Algorithm::HS512),
						) {
							Ok(data) => {
								let claims = data.claims;
								if claims.exp > timestamp_u64() {
									Box::pin(ready(Ok(claims)))
								} else {
									Box::pin(ready(Err(ErrorUnauthorized("Expired token!"))))
								}
							}
							Err(_) => {
								Box::pin(ready(Err(ErrorUnauthorized("Invalid token!"))))
							}
						}
					}
					#[cfg(feature = "basic-auth")]
					Some("Basic") => {
						let db = req.app_data::<Data<DatabaseWrapper>>().unwrap().clone();
						Box::pin(async_basic_auth(db.into_inner(), base64::decode(token).ok().and_then(|it| String::from_utf8(it).ok())))
					}
					_ => {
						Box::pin(ready(Err(ErrorUnauthorized("Invalid token!"))))
					}
				}
			}
			None => Box::pin(ready(Err(ErrorUnauthorized("Missing token!"))))
		}
	}
}