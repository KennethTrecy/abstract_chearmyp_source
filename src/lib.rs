#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Abstract Source
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `str_source`: Implements [AbstractSource] for &[str].
//! - `slice_u8_source`: Implements [AbstractSource] for `&[u8]` [slice].
//! - `vec_source_collection`: Implements [AbstractSourceCollection] for [Vec<T>].
//! - `no_std`: Uses the `core` crate instead of `std` crate.

#[cfg(feature = "no_std")]
extern crate alloc;

mod abstract_source;
mod abstract_source_collection;

pub use abstract_source::AbstractSource;
pub use abstract_source_collection::AbstractSourceCollection;
