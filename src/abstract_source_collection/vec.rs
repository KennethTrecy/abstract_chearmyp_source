#[cfg(feature = "no_std")]
use alloc::vec::Vec;

use crate::{AbstractSource, AbstractSourceCollection};

/// This is only available if `vec_source_collection` feature has been activated.
///
/// It implements [AbstractSourceCollection] for alloc::vec::[Vec] instead if `no_std` feature has
/// been activated.
impl<T> AbstractSourceCollection<T> for Vec<T>
where
	T: AbstractSource {
	fn get_source(&self, index: usize) -> Option<&T> {
		self.get(index)
	}

	fn add_source(&mut self, source: T) {
		self.push(source);
	}
}

#[cfg(test)]
mod t {
	use alloc::vec;
	use super::{Vec, AbstractSourceCollection};

	#[test]
	fn can_create_new_collection() {
		let collection = Vec::<&[u8]>::new();

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
		let mut collection = Vec::<&str>::new();

		collection.add_source("b");

		assert_eq!(collection, vec!["b"]);
	}
}
