use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

/// api status this should attach to any api response
#[derive(Serialize, Deserialize)]
pub struct ApiStatus {
	ok: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	error: Option<String>,
}

impl ApiStatus {
	/// Ok status return if api is ok
	pub fn ok() -> Self {
		Self {
			ok: true,
			error: None,
		}
	}

	/// Error return if api error occurred
	pub fn error(message: String) -> Self {
		Self {
			ok: false,
			error: Some(message),
		}
	}
}

impl From<StatusCode> for ApiStatus {
	fn from(code: StatusCode) -> Self {
		if code.is_success() {
			ApiStatus::ok()
		} else {
			ApiStatus::error(format!("{:?}", code))
		}
	}
}