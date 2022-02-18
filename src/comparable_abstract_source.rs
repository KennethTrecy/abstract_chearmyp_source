/// An abstraction for comparable source.
pub trait ComparableAbstractSource<T> {
	/// Checks if the string matches the needle at the targeted index. Equivalent to running
	/// `&source[n..].starts_with(m)` where `n` is the first argument indicating where to start
	/// checking and `m` is the second argument indicating needle to match.
	fn is_same_needle_at(&self, _: usize, _: T) -> bool;
}

#[macro_use]
#[cfg(test)]
mod test_implementation_macro;

#[cfg(any(
	feature = "str_comparable_to_u8",
	feature = "str_comparable_to_str"
))]
mod str;

#[cfg(any(
	feature = "slice_u8_comparable_to_u8",
	feature = "slice_u8_comparable_to_str"
))]
mod slice_u8;
