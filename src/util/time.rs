use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Duration;

/// get current timestamp as u64
#[inline]
pub fn timestamp_u64() -> u64 {
	timestamp_u128() as u64
}

/// get current timestamp
#[inline]
pub fn timestamp_u128() -> u128 {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

/// timestamp helper use to get relative timestamp from now
pub trait TimestampExt<T> {
	/// get relative timestamp from now result should return as `current_timestamp + self.milliseconds()`
	fn timestamp_from_now(&self) -> T;
}

impl TimestampExt<i64> for Duration {
	fn timestamp_from_now(&self) -> i64 {
		(timestamp_u64() as i64) + self.num_milliseconds()
	}
}

impl TimestampExt<u128> for std::time::Duration {
	fn timestamp_from_now(&self) -> u128 {
		timestamp_u128() + self.as_millis()
	}
}