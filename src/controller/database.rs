use std::env;
use std::ops::Deref;
use mongodb::{Database};
use mongodb::options::ClientOptions;
use anyhow::Result;
use super::user_controller;
use super::user_controller::UserController;

/// use to extract database in route handler
pub type DatabaseRef = actix_web::web::Data<DatabaseWrapper>;

/// Wrapper for database connection may have helper method to get collection and other..
#[derive(Clone)]
#[repr(transparent)]
pub struct DatabaseWrapper(pub Database);

impl DatabaseWrapper {
	/// get user controller with pre-configured collection
	pub fn users(&self) -> UserController {
		self.into()
	}
}

impl Deref for DatabaseWrapper {
	type Target = Database;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// this function will load config from env and connect to database
pub async fn init_database() -> Result<DatabaseWrapper> {
	let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
	let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set in .env");
	let options = ClientOptions::parse(&db_url).await?;
	let client = mongodb::Client::with_options(options)?;
	let db = DatabaseWrapper(client.database(db_name.as_str()));
	preload(&db).await?;
	Ok(db)
}

/// this function will call after database connected can be used to
/// + init data once application start
/// + create(ensure) index
async fn preload(db: &DatabaseWrapper) -> Result<()> {
	// put initialize here
	user_controller::init(&db).await?;
	Ok(())
}