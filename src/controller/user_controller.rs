use std::ops::Deref;
use mongodb::{Collection, IndexModel};
use crate::controller::database::DatabaseWrapper;
use crate::schema::User;
use anyhow::Result;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;

/// this function will call after connected to database
pub async fn init(db: &DatabaseWrapper) -> Result<()> {
	// create index in this block

	let controller = db.users();
	let index: IndexModel = IndexModel::builder()
		.keys(doc! {"username":1})
		.options(IndexOptions::builder().unique(true).build())
		.build();
	controller.create_index(index, None).await?;
	Ok(())
}

/// this struct is wrapper to `Collection<User>` should have function to help to manage user
#[repr(transparent)]
pub struct UserController(pub Collection<User>);

impl UserController {
	/// find user by username return None if not found
	pub async fn find_by_username(&self, username: impl AsRef<str>) -> Option<User> {
		self.0.find_one(doc! {"username":username.as_ref()}, None).await.ok()?
	}
}

/// # Example
/// ```rust
/// use actix_mongo_jwt_web_template::controller::database::DatabaseWrapper;
/// use actix_mongo_jwt_web_template::controller::user_controller::UserController;
/// let db: &DatabaseWrapper;
/// let uc: UserController = db.into();
/// ```
impl From<&DatabaseWrapper> for UserController {
	fn from(db: &DatabaseWrapper) -> Self {
		UserController(db.collection("users"))
	}
}

impl Deref for UserController {
	type Target = Collection<User>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}