/**
Difficulty: [Medium](https://leetcode.com/problems/3sum-closest/description/)

Finds the sum of three integers in `nums` that is closest to `target`.

This function first sorts the input vector `nums` to make it easier to find the
three integers. Then, it initializes the closest sum as the sum of the first three
elements in the sorted vector and computes its difference with the `target`.

It iterates through the vector, maintaining two pointers that move towards each
other to try different sums, updating the closest sum found so far whenever a
closer sum is found. If at any point the exact `target` sum is found, it returns
immediately.

# Parameters
- `nums`: A vector of integers in which to find the three integers.
- `target`: The target sum to approach.

# Returns

An integer representing the closest sum to `target` that can be obtained
by summing any three integers in `nums`.

# Example

```rust
use your_crate_name::three_sum_closest;

let nums = vec![-1, 2, 1, -4];
let target = 1;
let result = three_sum_closest(nums, target);

assert_eq!(result, 2);
*/
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
	let mut nums = nums;
	let nums_len = nums.len();
	let mut closest = nums[0..3].iter().fold(0, |acc, x| acc + x);
	let mut closest_diff = (target - closest).abs();

	nums.sort_unstable();

	for i in 0..(nums_len - 2) {
		let mut j = i + 1;
		let mut k = nums_len - 1;

		while j < k {
			let sum = nums[i] + nums[j] + nums[k];
			let curr_diff = (target - sum).abs();

			if curr_diff == 0 {
				return sum;
			}

			if curr_diff < closest_diff {
				closest = sum;
				closest_diff = curr_diff;
			}

			match sum < target {
				true => j += 1,
				false => k -= 1,
			};
		}
	}

	closest
}

#[cfg(test)]
mod tests {
	use super::three_sum_closest;

	#[test]
	fn test_example_1() {
		assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
	}

	#[test]
	fn test_example_2() {
		assert_eq!(three_sum_closest(vec![0, 2, 1, -3], 1), 0);
	}

	#[test]
	fn test_large_numbers() {
		assert_eq!(three_sum_closest(vec![1000, 2000, 3000], 6000), 6000);
	}

	#[test]
	fn test_negative_target() {
		assert_eq!(three_sum_closest(vec![1, 2, 3], -100), 6);
	}

	#[test]
	fn test_positive_target() {
		assert_eq!(three_sum_closest(vec![-1, -2, -3], 100), -6);
	}

	#[test]
	fn test_mixed_signs() {
		assert_eq!(three_sum_closest(vec![-1, 2, -3, 4], 3), 3);
	}
}
