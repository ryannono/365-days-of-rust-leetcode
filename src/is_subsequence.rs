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
	let s_bytes = s.as_bytes();
	let t_bytes = t.as_bytes();

	let mut s_i = 0;
	let mut t_i = 0;

	while s_i < s_bytes.len() && t_i < t_bytes.len() {
		if s_bytes[s_i] == t_bytes[t_i] {
			s_i += 1;
		}
		t_i += 1;
	}

	s_i == s_bytes.len()
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
