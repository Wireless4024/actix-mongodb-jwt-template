use std::ops::Deref;

use anyhow::bail;
use anyhow::Result;

use crate::manager::DatabaseWrapper;
use crate::schema::User;

/// this module contains middleware / from handle for actix
pub mod middleware;

/// login user using username and password
pub async fn login_by_username(db: impl Deref<Target=DatabaseWrapper>, username: &str, password: &str) -> Result<User> {
	if let Some(user) = db.users().find_by_username(username).await {
		if user.verify_password(password).await {
			return Ok(user);
		}
	}
	bail!("invalid username or password?")
}