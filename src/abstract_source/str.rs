use crate::AbstractSource;

/// This is only available if `str_source` feature has been activated.
impl<'a> AbstractSource for &'a str {
	type Slice = &'a str;

	fn forward_slice(&self, start_index: usize) -> Self::Slice {
		&self[start_index..]
	}

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn is_equal_at(&self, index: usize, byte: u8) -> bool {
		self.get(index..index+1)
			.unwrap_or("")
			.as_bytes()
			.get(0) == Some(&byte)
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

	#[test]
	fn should_be_equal() {
		let source = "d";
		let index = 0;
		let byte = 'd' as u8;

		let is_equal = source.is_equal_at(index, byte);

		assert_eq!(is_equal, true);
	}

	#[test]
	fn should_not_be_equal() {
		let source = "e";
		let index = 0;
		let byte = 'f' as u8;

		let is_equal = source.is_equal_at(index, byte);

		assert_eq!(is_equal, false);
	}

	#[test]
	fn may_not_compare_unreachable() {
		let source = "g";
		let index = 1;
		let byte = 'h' as u8;

		let is_equal = source.is_equal_at(index, byte);

		assert_eq!(is_equal, false);
	}
}
