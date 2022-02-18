use crate::AbstractSource;

/// An abstraction of source collection.
pub trait AbstractSourceCollection<T: AbstractSource> {
	/// Gets the source at the specified index.
	fn get_source(&self, _: usize) -> Option<&T>;

	/// Add the source into the collection.
	fn add_source(&mut self, _: T);
}

#[cfg(feature = "vec_source_collection")]
mod vec;
