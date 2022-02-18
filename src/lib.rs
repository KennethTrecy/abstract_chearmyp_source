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
//! - `str_comparable_to_u8`. Implements [ComparableAbstractSource] for &[str] allowing
//!   comparison(s) to [u8].
//! - `str_comparable_to_str`. Implements [ComparableAbstractSource] for &[str] allowing
//!   comparison(s) to &[str].
//! - `slice_u8_comparable_to_u8`. Implements [ComparableAbstractSource] for `&[u8]` [slice]
//!   allowing comparison(s) to [u8].
//! - `slice_u8_comparable_to_str`. Implements [ComparableAbstractSource] for `&[u8]` [slice]
//!   allowing comparison(s) to &[str].

#[cfg(feature = "no_std")]
extern crate alloc;

mod abstract_source;
mod abstract_source_collection;
mod comparable_abstract_source;

pub use abstract_source::AbstractSource;
pub use abstract_source_collection::AbstractSourceCollection;
pub use comparable_abstract_source::ComparableAbstractSource;
