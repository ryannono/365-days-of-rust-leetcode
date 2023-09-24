/// Dificulty: [Medium](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/description)
/// 
/// Returns the maximum number of vowels in a contiguous substring of length `k`
/// from the string `s`.
///
/// # Arguments
///
/// * `s` - A string from which the substring is to be extracted.
/// * `k` - The length of the substring to be considered.
///
/// # Examples
///
/// ```
/// let s = "examplestring".to_string();
/// let k = 4;
/// assert_eq!(max_vowels(s, k), 2); // "ampl" has 2 vowels: a, e
/// ```
///
/// # Note
///
/// The function will return 0 if the string `s` has length less than `k` or if
/// there are no vowels in the given substring length.
pub fn max_vowels(s: String, k: i32) -> i32 {
	let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
	let chars: Vec<char> = s.chars().collect();
	let mut max_vowels = 0;
	let mut current_vowels = 0;

	for i in 0..chars.len() {
		if vowels.contains(&chars[i]) {
			current_vowels += 1;
		}

		if i as i32 >= k && vowels.contains(&chars[i - k as usize]) {
				current_vowels -= 1;
		}

		max_vowels = std::cmp::max(max_vowels, current_vowels);
	}

	max_vowels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "examplestring".to_string();
        let k = 3;
        assert_eq!(max_vowels(s, k), 2);
    }

    #[test]
    fn test_example_2() {
        let s = "hello".to_string();
        let k = 2;
        assert_eq!(max_vowels(s, k), 1);
    }

    #[test]
    fn test_example_3() {
        let s = "aaee".to_string();
        let k = 2;
        assert_eq!(max_vowels(s, k), 2);
    }

    #[test]
    fn test_example_4() {
        let s = "aeiouaeiou".to_string();
        let k = 5;
        assert_eq!(max_vowels(s, k), 5);
    }
}
