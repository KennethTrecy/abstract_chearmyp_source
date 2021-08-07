#[cfg(feature = "no_std")]
use alloc::vec::Vec;

use crate::{AbstractSource, AbstractSourceCollection};

impl<T> AbstractSourceCollection for Vec<T>
where
	T: AbstractSource {
	type Source = T;

	fn new() -> Self {
		Vec::new()
	}

	fn get_source(&self, index: usize) -> Option<&Self::Source> {
		self.get(index)
	}

	fn add_source(&mut self, source: Self::Source) {
		self.push(source);
	}
}

#[cfg(test)]
mod t {
	use alloc::vec;
	use super::{Vec, AbstractSourceCollection};

	#[test]
	fn can_create_new_collection() {
		let collection = <Vec<&[u8]> as AbstractSourceCollection>::new();

		assert_eq!(collection, Vec::<&[u8]>::new());
	}

	#[test]
	fn can_get_source() {
		let collection = vec!["a"];

		let source = collection.get_source(0);

		assert_eq!(source, Some(&"a"));
	}

	#[test]
	fn can_add_source() {
		let mut collection = <Vec<&str> as AbstractSourceCollection>::new();

		collection.add_source("b");

		assert_eq!(collection, vec!["b"]);
	}
}
