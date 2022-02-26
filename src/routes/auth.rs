use actix_web::{HttpResponse, Responder, Scope, web, get, post};
use actix_web::web::{Json};
use serde::{Serialize, Deserialize};
use crate::auth::middleware::{login_as_token};
use crate::controller::database::{DatabaseRef};
use crate::schema::Jwt;

/// construct routes in current scope
/// parent scope should call this method to append routes
pub fn get_scope() -> Scope {
	web::scope("/auth")
		// route to /auth/login
		.service(login)
		// route to /auth/check
		.service(check)
}

/// use to receive login information from client
#[derive(Deserialize)]
struct LoginData {
	// you can change `username` to `email` or whatever
	username: String,
	password: String,
}

/// use to response token to client
#[derive(Serialize)]
struct LoginResponse {
	token: String,
}

/// this route will take username and password from request and response token back
/// ## Request
/// ```http
/// POST /auth/login
/// Content-Type: application/json
///
/// {"username":"username","password":"password"}
/// ```
/// ## Response
/// + 200 `{"token":"..jwt..token.."}`
/// + 401 if failed to verify username or password
#[post("/login")]
async fn login(Json(LoginData { username, password }): Json<LoginData>,db:DatabaseRef) -> impl Responder {
	if let Ok(token) = login_as_token(db.get_ref(),username.as_str(), password.as_str()).await {
		return HttpResponse::Ok().json(LoginResponse { token });
	}
	HttpResponse::Unauthorized().finish()
}

/// this route use to check token (have nothing because it already handles in jwt)
/// ## Request
/// ```http
/// GET /auth/check
/// Authorization: Bearer "jwt..token"
/// ```
/// ## Response
/// + 200 if token is valid
/// + 401 if token is expired or invalid
#[get("/check")]
async fn check(_: Jwt) -> impl Responder { "" }