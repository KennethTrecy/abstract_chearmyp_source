use crate::ComparableAbstractSource;

/// This is only available if `slice_u8_comparable_to_u8` feature has been activated.
impl ComparableAbstractSource<u8> for &[u8] {
	fn is_same_needle_at(&self, index: usize, needle: u8) -> bool {
		self.get(index) == Some(&needle)
	}
}

#[cfg(test)]
test_implementation!{
	should_contain_needle: &b"a"[..], at 0 is 'a' as u8,
	should_not_contain_needle: &b"b"[..], at 0 is 'c' as u8,
	may_not_compare_unreachable: &b"d"[..], at 2 is 'd' as u8
}
