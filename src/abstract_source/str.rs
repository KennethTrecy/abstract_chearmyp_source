use crate::AbstractSource;

/// This is only available if `str_source` feature has been activated.
impl<'a> AbstractSource for &'a str {
	fn forward_slice(self, start_index: usize) -> Self {
		&self[start_index..]
	}

	fn slice(self, start_index: usize, stop_index: usize) -> Self {
		&self[start_index..stop_index]
	}

	fn is_empty_at(&self, index: usize) -> bool {
		index >= self.len()
	}
}

#[cfg(test)]
mod t {
	use super::AbstractSource;

	#[test]
	fn can_forward_slice() {
		let source = "ab";
		let index = 1;

		let slice = source.forward_slice(index);

		assert_eq!(slice, &source[index..]);
	}

	#[test]
	fn should_be_empty() {
		let source = "";

		let is_empty = source.is_empty();

		assert_eq!(is_empty, true);
	}

	#[test]
	fn should_not_be_empty() {
		let source = "c";

		let is_empty = source.is_empty();

		assert_eq!(is_empty, false);
	}
}
