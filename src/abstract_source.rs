/// An abstraction of source.
pub trait AbstractSource {
	/// Creates a forward slice. Equivalent to running `&source[n..]` where `n` is the first argument
	/// indicating the index where to start slicing.
	fn forward_slice(self, _: usize) -> Self;

	/// Creates a slice. Equivalent to running `&source[n..m]` where `n` is the first argument
	/// indicating the index where to start slicing and `m` indicating the index where to stop
	/// slicing.
	fn slice(self, _: usize, _: usize) -> Self;

	/// Check if the source if it still has contents at certain offset.
	fn is_empty_at(&self, _: usize) -> bool;
}

#[cfg(feature = "str_source")]
mod str;

#[cfg(feature = "slice_u8_source")]
mod slice_u8;
