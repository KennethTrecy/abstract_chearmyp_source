/// An abstraction of source.
pub trait AbstractSource {
	/// Creates a forward slice. Equivalent to running `&source[n..]` where `n` is the first argument
	/// indicating the index where to start slicing.
	fn forward_slice(&self, _: usize) -> &Self;

	/// Check if the source still has contents
	fn is_empty(&self) -> bool;

	/// Checks if the has a byte at the targeted index. Equivalent to running `source[n] == m` where
	/// `n` is the first argument indicating the index to check and `m` is the second argument
	/// indicating the byte it will be compared to.
	fn is_equal_at(&self, _: usize, _: u8) -> bool;
}
