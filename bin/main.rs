use std::env;
use std::str::FromStr;
use actix_cors::Cors;
use actix_web::http::KeepAlive;
use actix_web::{App, HttpServer};
use actix_web::middleware::DefaultHeaders;
use actix_web::web::Data;
use anyhow::Result;
use actix_mongo_jwt_web_template::{
	routes::auth,
	controller::database::init_database,
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
		if let Ok(hosts) = env::var("CORS_HOSTS") {
			if !hosts.is_empty() {
				for host in hosts.as_str().split(",") {
					cors = cors.allowed_origin(host);
				}
			} else {
				cors = Cors::permissive();
			}
		} else {
			cors = Cors::permissive();
		}

		let mut app: App<_> = App::new()
			.wrap(DefaultHeaders::new()
				.add(("X-Frame-Options", "DENY"))
				.add(("Referrer-Policy", "no-referrer")))
			.wrap(cors)
			.app_data(Data::new(database.clone()));

		app = app.service(auth::get_scope());
		app
	});
	let server = if let Ok(socket) = std::env::var("BIND_SOCKET") {
		let mut sock = if !socket.is_empty() { server.bind_uds(socket)? } else { server };
		let may_bind_ip = match std::env::var("BIND_SOCKET_ONLY").as_deref() {
			Ok("1") | Ok("true") | Ok("y") => true,
			_ => false
		};
		if may_bind_ip {
			sock = sock.bind(std::env::var("BIND").expect("please set `BIND` in environment variable"))?
		}
		sock
	} else {
		server.bind(std::env::var("BIND").expect("please set `BIND` in environment variable"))?
	};

	server.keep_alive(
		keep_alive().unwrap_or(KeepAlive::Timeout(std::time::Duration::from_secs(30)))
	).run().await?;
	Ok(())
}

/// helper function
fn keep_alive() -> Option<KeepAlive> {
	let alive = std::env::var("KEEP_ALIVE").ok()?;
	if alive.is_empty() { return None; }

	let secs = std::time::Duration::from_secs(u64::from_str(alive.as_str()).ok()?);

	Some(KeepAlive::Timeout(secs))
}