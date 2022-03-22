/// get env var as string it will ensure value is not empty string
#[inline]
pub fn env(key: &str) -> Option<String> {
	std::env::var(key).ok().and_then(|it| if it.is_empty() { None } else { Some(it) })
}

/// get raw env var as Vec<u8>
#[inline]
pub fn raw_env(key: &str) -> Option<Vec<u8>> {
	#[cfg(target_os = "linux")]
	{
		use std::os::unix::prelude::OsStringExt;
		Some(std::env::var_os(key)?.into_vec())
	}
	#[cfg(not(target_os = "linux"))]
	{
		Some(std::env::var_os(key)?.to_string_lossy().as_bytes().to_vec())
	}
}