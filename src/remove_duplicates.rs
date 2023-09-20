trait ExtString {
	fn cmp_last(&self, compare_val: char) -> bool;
}

impl ExtString for String {
	/// The function `cmp_top` compares the last character of a string with a given value.
	///
	/// Arguments:
	///
	/// * `compare_val`: The `compare_val` parameter is the value that will be compared to the last element
	/// of the data structure.
	///
	/// Returns:
	///
	/// Whether the top matches the passed value.
	fn cmp_last(&self, compare_val: char) -> bool {
		self.chars().nth_back(0)
			.map_or(false, |last_char| last_char == compare_val)
	}
}

/**
Difficulty: [Easy](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/description/)

Removes adjacent duplicate characters from a string.

This function iterates over each character in the input string and uses a stack
to keep track of the characters seen. If a character is found to be the same as 
the one at the top of the stack, all adjacent duplicates are removed from the 
stack. Finally, the characters in the stack are collected to form a new string
without adjacent duplicates.

# Parameters

- `s`: The input string from which adjacent duplicates need to be removed.

# Returns

A new string where all adjacent duplicate characters have been removed.

# Example

```rust
use your_crate_name::remove_duplicates;

let input = String::from("abbaca");
let output = remove_duplicates(input);

assert_eq!(output, "ca");
*/
pub fn remove_duplicates(s: String) -> String {
	let mut result_str = String::new();

	for curr_char in s.chars() {
		match result_str.cmp_last(curr_char) {
			true => {
				while result_str.cmp_last(curr_char) {
					result_str.pop();
				}
			}

			false => result_str.push(curr_char),
		}
	}

	result_str
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
