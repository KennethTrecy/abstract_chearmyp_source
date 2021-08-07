#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Abstract Source
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `str_source`: Implements [AbstractSource] for [str].
//! - `slice_u8_source`: Implements [AbstractSource] for `&[u8]` [slice].
//! - `no_std`: Uses the `core` crate instead of `std` crate.

mod abstract_source;

pub use abstract_source::AbstractSource;
