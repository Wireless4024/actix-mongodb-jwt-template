use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use mongodb::{Collection, IndexModel};
use mongodb::bson::{doc, Document};
use mongodb::options::IndexOptions;

use crate::manager::DatabaseWrapper;

/// this module contains user repository use to manage user from database
pub mod user_repo;
pub use user_repo::UserRepository;

/// base repository trait provide basic functional of database repository
pub trait Repository<T, F: Deref<Target=DatabaseWrapper>>: From<F> + Deref<Target=Collection<T>>
	where T: 'static {
	/// ensure index of single field is created in collection
	#[inline]
	fn ensure_index_single(&self, field: impl AsRef<str>) -> Pin<Box<dyn Future<Output=anyhow::Result<()>>>> {
		self.ensure_index(doc! {field.as_ref():1}, IndexOptions::default())
	}

	/// ensure index of single field is created in collection with option configuration closure
	/// # Example
	/// ```rust
	/// use actix_mongo_jwt_web_template::repository::Repository;
	/// use actix_mongo_jwt_web_template::schema::User;
	/// let db: mongodb::Database;
	/// let repo:impl Repository<User,_> = db.users();
	/// repo.ensure_index_single_option("username", |cfg| { cfg.unique = Some(true) }).await?;
	/// ```
	#[inline]
	fn ensure_index_single_option(&self, field: impl AsRef<str>, cfg: fn(&mut IndexOptions) -> ()) -> Pin<Box<dyn Future<Output=anyhow::Result<()>>>> {
		let mut option = IndexOptions::default();
		cfg(&mut option);
		self.ensure_index(doc! {field.as_ref():1}, option)
	}

	/// ensure index is available in database
	/// this is helper function due intellij-rust have no idea about their macro
	fn ensure_index(&self, index: Document, option: IndexOptions) -> Pin<Box<dyn Future<Output=anyhow::Result<()>>>> {
		// cloning is very cheap
		let collection = self.deref().clone();
		//	IndexOptions::builder().unique(true).build()
		Box::pin(async move {
			let index: IndexModel = IndexModel::builder().keys(index).options(option).build();
			collection.create_index(index, None).await?;
			anyhow::Result::Ok(())
		})
	}
}