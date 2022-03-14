use std::ops::Deref;

use anyhow::Result;
use mongodb::bson::doc;
use mongodb::Collection;

use crate::manager::DatabaseWrapper;
use crate::repository::Repository;
use crate::schema::User;

/// this function will call after connected to database
pub async fn init(db: &DatabaseWrapper) -> Result<()> {
	// create index in this block

	let controller = db.users();
	// builder style intellij can't highlight them
	controller.ensure_index_single_option("username", |cfg| { cfg.unique = Some(true) }).await?;
	Ok(())
}

/// this struct is wrapper to `Collection<User>` should have function to help to manage user
#[repr(transparent)]
pub struct UserRepository(pub Collection<User>);

impl UserRepository {
	/// find user by username return None if not found
	pub async fn find_by_username(&self, username: impl AsRef<str>) -> Option<User> {
		self.0.find_one(doc! {"username":username.as_ref()}, None).await.ok()?
	}
}

impl Repository<User, &DatabaseWrapper> for UserRepository {}

/// # Example
/// ```rust
/// use actix_mongo_jwt_web_template::manager::DatabaseWrapper;
/// use actix_mongo_jwt_web_template::repository::UserRepository;
/// let db: &DatabaseWrapper;
/// let uc: UserRepository = db.into();
/// ```
impl From<&DatabaseWrapper> for UserRepository {
	fn from(db: &DatabaseWrapper) -> Self {
		UserRepository(db.collection("users"))
	}
}

impl Deref for UserRepository {
	type Target = Collection<User>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}