use crate::ComparableAbstractSource;

/// This is only available if `slice_u8_comparable_to_str` feature has been activated.
impl ComparableAbstractSource<&str> for &[u8] {
	fn is_same_needle_at(&self, index: usize, needle: &str) -> bool {
		self.get(index..).map_or_else(|| false, |source| source.starts_with(needle.as_bytes()))
	}
}

#[cfg(test)]
test_implementation!{
	should_contain_needle: "a", at 0 is "a",
	should_not_contain_needle: "b", at 0 is "c",
	may_not_compare_unreachable: "d", at 2 is "d"
}
