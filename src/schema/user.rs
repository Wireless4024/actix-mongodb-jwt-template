use bcrypt::{hash, verify};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

/// this struct store user information
#[derive(Serialize, Deserialize)]
pub struct User {
	_id: ObjectId,
	username: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	password: Option<String>,
}

impl User {
	/// create new user form username and generate _id automatically
	pub fn new(username: String) -> Self {
		Self {
			_id: Default::default(),
			username,
			password: None,
		}
	}

	/// get user id as reference
	pub fn id_ref(&self) -> &ObjectId {
		&self._id
	}

	/// check if user's password is correct
	pub async fn verify_password(&self, password: impl AsRef<[u8]>) -> bool {
		if let Some(hash) = &self.password {
			let password = password.as_ref().to_vec();
			let hash = hash.to_string();
			// prevent verify from blocking executor
			tokio_rayon::spawn_fifo(move || {
				verify(password, hash.as_str()).unwrap_or_default()// default: false
			}).await
		} else {
			false
		}
	}

	/// change user's password to new password
	pub async fn set_password(&mut self, password: impl AsRef<[u8]>) -> bool {
		let password = password.as_ref().to_vec();

		// prevent hashing from blocking executor
		if let Ok(hash) = tokio_rayon::spawn(move || { hash(password, 12) }).await {
			self.password = Some(hash);
			true
		} else {
			false
		}
	}
}