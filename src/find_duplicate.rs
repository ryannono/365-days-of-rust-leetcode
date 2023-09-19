use std::collections::HashSet;

/**
Difficulty: [Medium](https://leetcode.com/problems/find-the-duplicate-number/?envType=daily-question&envId=2023-09-19)

Finds the first duplicate element in the given vector of integers.

This function iterates over the input vector and uses a hash set to keep track
of seen numbers. If a duplicate is found, it is immediately returned. If no
duplicate is found, it returns 0.

# Arguments

* `nums` - A vector of i32 integers. The input vector to search for duplicates.

# Returns

* An `i32` - The first duplicate element found in the vector. If no duplicate
  is found, it returns 0.

# Examples

```
# use your_crate::find_duplicate;
assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
assert_eq!(find_duplicate(vec![1, 2, 3, 4]), 0);
```
*/
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
	let mut seen: HashSet<i32> = HashSet::new();

	for num in nums {
		if !seen.insert(num) {
			return num;
		}
	}

	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_finds_a_duplicate() {
		assert_eq!(find_duplicate(vec![1, 2, 3, 4, 2]), 2);
	}

	#[test]
	fn it_finds_the_first_duplicate() {
		assert_eq!(find_duplicate(vec![5, 1, 3, 5, 2, 3]), 5);
	}

	#[test]
	fn it_handles_zero_as_a_valid_duplicate() {
		assert_eq!(find_duplicate(vec![0, 1, 0]), 0);
	}

	#[test]
	fn it_handles_negative_numbers() {
		assert_eq!(find_duplicate(vec![-1, -2, -3, -1]), -1);
	}

	#[test]
	fn it_handles_larger_vectors() {
		let large_vector: Vec<i32> = (0..1_000).chain(vec![999]).collect();
		assert_eq!(find_duplicate(large_vector), 999);
	}
}
