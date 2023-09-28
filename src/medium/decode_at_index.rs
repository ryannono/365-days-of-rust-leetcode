/// Difficulty: [Medium](https://leetcode.com/problems/decoded-string-at-index/description/?envType=daily-question&envId=2023-09-27)
///
/// Decodes a length-encoded string and returns the character found at the specified
/// index `k`.
///
/// The encoding is defined such that each letter is followed by a digit which
/// indicates the number of times it should be repeated in the decoded string.
///
/// # Arguments
///
/// * `s` - The encoded string.
/// * `k` - The 1-based index to retrieve the character from the decoded string.
///
/// # Examples
///
/// ```
/// assert_eq!(decode_at_index("a2b3c4".to_string(), 5), "b".to_string());
/// // The decoded string is "aabbbcccc", and the character at index 5 is 'b'.
/// ```
///
/// # Note
///
/// * If the input string is not correctly formatted or the index `k` is out of
///   bounds, the function will return an empty string.
pub fn decode_at_index(s: String, k: i32) -> String {
	let mut k = k as i64;
	let mut decoded_len =
		s.chars().fold(0, |acc: i64, ch| match ch.to_digit(10) {
			Some(digit) => acc * digit as i64,
			None => acc + 1,
		});

	for ch in s.chars().rev() {
		if k == 0 || k == decoded_len {
			match ch.is_numeric() {
				true => continue,
				false => return ch.to_string(),
			}
		}

		match ch.to_digit(10) {
			Some(digit) => {
				decoded_len /= digit as i64;
				k %= decoded_len;
			}

			None => decoded_len -= 1,
		}
	}

	String::new()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(
			decode_at_index("leet2code3".to_string(), 10),
			"o".to_string()
		);
	}

	#[test]
	fn test_2() {
		assert_eq!(decode_at_index("ha22".to_string(), 5), "h".to_string());
	}

	#[test]
	fn test_3() {
		assert_eq!(
			decode_at_index("a2345678999999999999999".to_string(), 1),
			"a".to_string()
		);
	}

	#[test]
	fn test_4() {
		assert_eq!(
			decode_at_index("y959q969u3hb22odq595".to_string(), 222280369),
			"y".to_string()
		);
	}

	#[test]
	fn test_5() {
		assert_eq!(
			decode_at_index("a2b3c4d5e6f7g8h9".to_string(), 10),
			"c".to_string()
		);
	}
}
