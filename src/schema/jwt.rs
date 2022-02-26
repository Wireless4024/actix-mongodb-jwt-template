use serde::{Serialize, Deserialize};

/// Claims for JWT
#[derive(Serialize, Deserialize)]
pub struct Jwt {
	pub(crate) sub: String,
	pub(crate) exp: u64,
}