/**
Difficulty: [Medium](https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/)

Generates all possible letter combinations that the provided digits could represent
on a phone's dial pad.

Given a string containing digits from `2-9` inclusive, return all possible letter
combinations that the number could represent. The mapping of digits to letters is
the same as on a telephone button layout.

# Parameters

- `digits`: A string of digits. The function does not handle digits `0` and `1`.

# Returns

A vector of strings representing all the letter combinations that can be formed by
the input digits.

# Example

```rust
use your_crate_name::letter_combinations;

let digits = String::from("23");
let result = letter_combinations(digits);

assert_eq!(result, vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
*/
pub fn letter_combinations(digits: String) -> Vec<String> {
	if digits.is_empty() {
		return vec![];
	}

	let chars = vec![
		vec![],
		vec![],
		vec!['a', 'b', 'c'],
		vec!['d', 'e', 'f'],
		vec!['g', 'h', 'i'],
		vec!['j', 'k', 'l'],
		vec!['m', 'n', 'o'],
		vec!['p', 'q', 'r', 's'],
		vec!['t', 'u', 'v'],
		vec!['w', 'x', 'y', 'z'],
	];

	let get_chars =
		|digit: char| chars.get(digit.to_digit(10).unwrap() as usize).unwrap();

	let char_sets: Vec<&Vec<char>> = digits.chars().map(get_chars).collect();

	combine_chars(String::new(), &char_sets)
}

/**
Recursively generates combinations of characters from a set of characters.

This function works by appending characters from the first set to the current string
and recursively calling itself for the remaining sets.

Parameters
curr_str: The current combination of characters being formed.
remaining_sets: A slice of vectors containing sets of characters to be combined.
Returns
A vector of strings representing all the combinations formed using the characters in
the sets.

Note: This function is intended to be used internally by letter_combinations.
*/
fn combine_chars(
	curr_str: String,
	remaining_sets: &[&Vec<char>],
) -> Vec<String> {
	if remaining_sets.is_empty() {
		return vec![curr_str];
	}

	let mut combinations = Vec::new();

	for &c in remaining_sets[0] {
		let new_curr_str = format!("{}{}", curr_str, c);
		let new_combinations =
			combine_chars(new_curr_str, &remaining_sets[1..]);
		combinations.extend(new_combinations);
	}

	combinations
}

#[cfg(test)]
mod tests {
	use super::letter_combinations;

	#[test]
	fn test_example_1() {
		let digits = "23".to_string();
		let mut expected: Vec<String> =
			vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
				.iter()
				.map(|&s| s.to_string())
				.collect();
		let mut result = letter_combinations(digits);
		result.sort();
		expected.sort();
		assert_eq!(result, expected);
	}

	#[test]
	fn test_empty() {
		let digits = "".to_string();
		let expected: Vec<String> = vec![];
		assert_eq!(letter_combinations(digits), expected);
	}

	#[test]
	fn test_single_digit() {
		let digits = "2".to_string();
		let mut expected: Vec<String> =
			vec!["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
		let mut result = letter_combinations(digits);
		result.sort();
		expected.sort();
		assert_eq!(result, expected);
	}

	#[test]
	fn test_digits_with_four_chars() {
		let digits = "27".to_string();
		let mut expected: Vec<String> = vec![
			"ap", "aq", "ar", "as", "bp", "bq", "br", "bs", "cp", "cq", "cr",
			"cs",
		]
		.iter()
		.map(|&s| s.to_string())
		.collect();
		let mut result = letter_combinations(digits);
		result.sort();
		expected.sort();
		assert_eq!(result, expected);
	}
}
