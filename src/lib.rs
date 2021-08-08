#![cfg_attr(feature = "no_std", no_std)]

//! # Abstract Chearmyp Source
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `str_source`: Implements [AbstractSource] for &[str].
//! - `slice_u8_source`: Implements [AbstractSource] for `&[u8]` [slice].
#![cfg_attr(
	feature = "no_std",
	doc = "- `vec_source_collection`: Implements [AbstractSourceCollection] for [alloc::vec::Vec].",
)]
#![cfg_attr(
	not(feature = "no_std"),
	doc = "- `vec_source_collection`: Implements [AbstractSourceCollection] for [Vec].",
)]
//!- `no_std`: Uses the `core` crate instead of `std` crate.

#[cfg(feature = "no_std")]
extern crate alloc;

mod abstract_source;
mod abstract_source_collection;

pub use abstract_source::AbstractSource;
pub use abstract_source_collection::AbstractSourceCollection;
