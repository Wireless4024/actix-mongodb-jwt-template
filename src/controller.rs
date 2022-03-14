use actix_web::Scope;

/// contains routing to authentication
pub mod auth_controller;
pub use auth_controller::AuthController;

/// Base function for controller
pub trait Controller {
	/// this function use to create routing to the controller
	fn create_scope() -> Scope;
}