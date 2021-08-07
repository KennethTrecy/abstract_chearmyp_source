use crate::AbstractSource;

/// An abstraction of source collection.
pub trait AbstractSourceCollection : IntoIterator {
	/// The type of source the collection contains.
	type Source: AbstractSource;

	/// Creates an empty source collection.
	fn new() -> Self;

	/// Gets the source at the specified index.
	fn get_source(&self, _: usize) -> Option<&Self::Source>;
}
