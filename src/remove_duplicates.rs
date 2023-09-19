trait CharStack<T: PartialEq> {
	fn cmp_top(&self, compare_val: T) -> bool;
}

impl<T: PartialEq> CharStack<T> for Vec<T> {
	/// The function `cmp_top` compares the last element of a collection with a given value.
	///
	/// Arguments:
	///
	/// * `compare_val`: The `compare_val` parameter is the value that will be compared to the last element
	/// of the data structure.
	///
	/// Returns:
	///
	/// Whether the top matches the passed value.
	fn cmp_top(&self, compare_val: T) -> bool {
		self.last()
			.map_or(false, |last_char| *last_char == compare_val)
	}
}

pub fn remove_duplicates(s: String) -> String {
	let mut char_stack: Vec<char> = vec![];

	for curr_char in s.chars() {
		match char_stack.cmp_top(curr_char) {
			true => {
				while char_stack.cmp_top(curr_char) {
					char_stack.pop();
				}
			}

			false => char_stack.push(curr_char),
		}
	}

	char_stack.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn test_no_duplicates() {
        assert_eq!(remove_duplicates("abcdef".to_string()), "abcdef");
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(remove_duplicates("abb".to_string()), "a");
    }

    #[test]
    fn test_non_consecutive_duplicates() {
        assert_eq!(remove_duplicates("abba".to_string()), "");
    }

    #[test]
    fn test_mixed_duplicates() {
        assert_eq!(remove_duplicates("abbaca".to_string()), "ca");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(remove_duplicates("".to_string()), "");
    }
}
