
/**
Difficulty: [Easy](https://leetcode.com/problems/repeated-substring-pattern/solutions/3938737/c-uses-sieve-divisor-1-line-methods-solving-string-hard-easy/)

Checks if a string can be constructed by taking a substring of itself
and repeating it.

The function follows these conditions to check if the string `s` can be made
by repeating a substring:
1. Length of `s` must be 2 or greater.
2. The substring can be of length from 1 to s.len()/2.
3. The string `s` must start with the substring.
4. The end of the substring must mark the start of the next substring.
5. The length of `s` divided by the length of the substring must be an integer.

# Parameters

- `s`: The input string to check for the repeated substring pattern.

# Returns

A boolean value indicating whether the input string can be formed by
repeating a substring of itself.

# Example

```rust
use your_crate_name::repeated_substring_pattern;

let input = String::from("abab");
let output = repeated_substring_pattern(input);

assert_eq!(output, true);
*/
pub fn repeated_substring_pattern(s: String) -> bool {
	let s_len = s.len();

	if s_len < 2 {
		return false;
	};

	for i in 0..(s_len / 2) {
		let curr_substring_len = i + 1;

		if s_len % curr_substring_len == 0 {
			let curr_substring = &s[0..curr_substring_len];
			let mut curr_search_index = curr_substring_len;
			let mut count = 1;

			while curr_search_index < s_len
				&& s[curr_search_index..].starts_with(curr_substring)
			{
				count += 1;
				curr_search_index += curr_substring_len;
			}

			if count == s_len / curr_substring_len {
				return true;
			}
		}
	}

	false
}

#[cfg(test)]
mod tests {
    use super::repeated_substring_pattern;

    #[test]
    fn test_repeated_pattern() {
        assert!(repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn test_not_repeated_pattern() {
        assert!(!repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn test_single_char_repeated() {
        assert!(repeated_substring_pattern("aaaa".to_string()));
    }

    #[test]
    fn test_long_repeated_pattern() {
        assert!(repeated_substring_pattern("abcabcabc".to_string()));
    }

    #[test]
    fn test_empty_string() {
        assert!(!repeated_substring_pattern("".to_string()));
    }

    #[test]
    fn test_single_character() {
        assert!(!repeated_substring_pattern("a".to_string()));
    }
}
