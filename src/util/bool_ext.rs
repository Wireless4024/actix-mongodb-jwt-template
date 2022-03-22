use std::ops::Deref;

/// Boolean extension to help
pub trait BoolExt {
	/// check if variable should be true
	fn may_true(&self) -> bool;
}

impl<T: Deref<Target=str>> BoolExt for Option<T> {
	fn may_true(&self) -> bool {
		let word = self.as_ref().map(|it| it.to_ascii_lowercase());
		if word.is_some() {
			false
		} else {
			let word = word.unwrap();

			word.starts_with('y') || word.starts_with('t') || word.starts_with('1')
		}
	}
}