/**
Difficulty: [Easy](https://leetcode.com/problems/is-subsequence/description/)

Determines if one string is a subsequence of another.

A subsequence contains all the elements of the target sequence, but not
necessarily consecutively. This function checks if the string `s` is a
subsequence of string `t`.

# Parameters

- `s`: The string whose subsequence property is to be checked.
- `t`: The reference string where the subsequence is potentially located.

# Returns

A boolean value indicating whether `s` is a subsequence of `t`.

# Example

use your_crate_name::is_subsequence;

let result = is_subsequence(String::from("abc"), String::from("ahbgdc"));
assert_eq!(result, true);

let result = is_subsequence(String::from("axc"), String::from("ahbgdc"));
assert_eq!(result, false);
*/
pub fn is_subsequence(s: String, t: String) -> bool {
	let mut s_chars = s.chars();
	let mut curr_s_char = s_chars.next();

	for t_char in t.chars() {
		if curr_s_char.is_some_and(|s_char| s_char == t_char) {
			curr_s_char = s_chars.next();
		}
	}

	curr_s_char.is_none()
}

#[cfg(test)]
mod tests {
	use super::is_subsequence;

	#[test]
	fn test_subsequence() {
		assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()));
	}

	#[test]
	fn test_not_subsequence() {
		assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()));
	}

	#[test]
	fn test_empty_s() {
		assert!(is_subsequence("".to_string(), "ahbgdc".to_string()));
	}

	#[test]
	fn test_empty_t() {
		assert!(!is_subsequence("abc".to_string(), "".to_string()));
	}

	#[test]
	fn test_empty_s_and_t() {
		assert!(is_subsequence("".to_string(), "".to_string()));
	}

	#[test]
	fn test_s_longer_than_t() {
		assert!(!is_subsequence("abcdef".to_string(), "abc".to_string()));
	}

	#[test]
	fn test_single_char_s_and_t() {
		assert!(is_subsequence("a".to_string(), "a".to_string()));
	}

	#[test]
	fn test_single_char_s_not_in_t() {
		assert!(!is_subsequence("a".to_string(), "b".to_string()));
	}
}
