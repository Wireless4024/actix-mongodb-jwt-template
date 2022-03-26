use std::str::FromStr;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::http::KeepAlive;
use actix_web::middleware::DefaultHeaders;
use actix_web::web::Data;
use anyhow::Result;

use actix_mongo_jwt_web_template::{
	controller::{AuthController, Controller},
	manager::init_database,
	util::{
		bool_ext::BoolExt,
		env::env,
	},
	web::error::ApiStatus,
};

#[actix_rt::main]
async fn main() -> Result<()> {
	dotenv::dotenv().ok();
	tracing_subscriber::fmt::init();

	let database = init_database().await?;

	let server = HttpServer::new(move || {
		let mut cors = Cors::default()
			.allow_any_header()
			.allow_any_method()
			.supports_credentials();
		if let Some(hosts) = env("HTTP_CORS_HOSTS") {
			for host in hosts.as_str().split(",") {
				cors = cors.allowed_origin(host);
			}
		} else {
			cors = Cors::permissive();
		}

		let mut app: App<_> = App::new()
			.wrap(DefaultHeaders::new()
				.add(("X-Frame-Options", "DENY"))// deny loading in iframe
				.add(("Referrer-Policy", "no-referrer")))
			.wrap(cors)
			.app_data(Data::new(database.clone()));

		app = app.service(AuthController::create_scope())
		         .default_service(web::route().to(not_found));
		app
	});
	let server = if let Some(socket) = env("HTTP_BIND_SOCKET") {
		let mut sock = server.bind_uds(socket)?;
		let may_bind_ip = env("HTTP_BIND_SOCKET_ONLY").may_true();
		if may_bind_ip {
			sock = sock.bind(env("HTTP_BIND").expect("please set `HTTP_BIND` in environment variable"))?
		}
		sock
	} else {
		server.bind(env("HTTP_BIND").expect("please set `HTTP_BIND` in environment variable"))?
	};

	server.keep_alive(
		keep_alive().unwrap_or(KeepAlive::Timeout(std::time::Duration::from_secs(30)))
	).run().await?;
	Ok(())
}

// not found handler this will response as json error
async fn not_found() -> HttpResponse {
	HttpResponse::NotFound().json(ApiStatus::error("Not Found".to_string()))
}

/// helper function
fn keep_alive() -> Option<KeepAlive> {
	let alive = env("HTTP_KEEP_ALIVE")?;
	if alive.is_empty() { return None; }

	let secs = std::time::Duration::from_secs(u64::from_str(alive.as_str()).ok()?);

	Some(KeepAlive::Timeout(secs))
}