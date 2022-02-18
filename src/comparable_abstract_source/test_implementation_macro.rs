#[macro_export]
macro_rules! test_implementation {
	(
		should_contain_needle: $first_source:expr, at $first_index:literal is $first_needle:expr,
		should_not_contain_needle:
			$second_source:expr, at $second_index:literal is $second_needle:expr,
		may_not_compare_unreachable: $third_source:expr, at $third_index:literal is $third_needle:expr
	) => {
		#[cfg(test)]
		mod t {
			use super::ComparableAbstractSource;

			test_implementation!{
				should_contain_needle using $first_source,
				if the needle at $first_index is $first_needle
			}

			test_implementation!{
				should_not_contain_needle using $second_source,
				if the needle at $second_index is $second_needle
			}

			test_implementation!{
				may_not_compare_unreachable using $third_source,
				if the needle at $third_index is $third_needle
			}
		}
	};

	(should_contain_needle using $source:expr, if the needle at $index:literal is $needle:expr) => {
		test_implementation!(should_contain_needle, $source, $index, $needle, true);
	};

	(should_not_contain_needle using $source:expr, if the needle at $index:literal is $needle:expr) => {
		test_implementation!(should_not_contain_needle, $source, $index, $needle, false);
	};

	(
		may_not_compare_unreachable
		using $source:expr,
		if the needle at $index:literal is $needle:expr) => {
		test_implementation!(may_not_compare_unreachable, $source, $index, $needle, false);
	};

	($name:ident, $source:expr, $index:literal, $needle:expr, $result:literal) => {
		#[test]
		fn $name() {
			let source = $source;
			let index = $index;
			let needle = $needle;

			let is_same = source.is_same_needle_at(index, needle);

			assert_eq!(is_same, $result);
		}
	}
}
