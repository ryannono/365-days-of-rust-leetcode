pub enum Side {
	Left,
	Right,
}

/**
Finds all the unique triplets in the array `nums` which sum up to zero.

This function first sorts the input vector `nums` in non-decreasing order. It then 
iterates through the sorted `nums`, maintaining two pointers, `left_i` and `right_i`, 
that move towards each other to explore different triplet combinations. A closure 
named `get_next_i` is defined within the function to help move the pointers past 
any duplicates and avoid duplicate triplets in the output.

# Parameters
- `nums`: A vector of integers where the triplets are to be found.

# Returns

A vector of vectors, where each inner vector represents a triplet that sums up to 
zero. If no such triplets are found, returns an empty vector.

# Example

```rust
use your_crate_name::three_sum;

let nums = vec![-1, 0, 1, 2, -1, -4];
let result = three_sum(nums);

assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

*/
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
	let mut nums = nums;
	let nums_len = nums.len();
	let mut triplets: Vec<Vec<i32>> = vec![];

	nums.sort_unstable();

	// Closure to get the next index to consider for either side
	let get_next_i = |side: Side, left_i: usize, right_i: usize| match side {
		Side::Left => (left_i..right_i)
			.find(|&future_left_i| nums[future_left_i] != nums[left_i])
			.unwrap_or(right_i),

		Side::Right => (left_i..right_i)
			.rfind(|&future_right_i| nums[future_right_i] != nums[right_i])
			.unwrap_or(left_i),
	};

	for i in 0..(nums_len - 2) {
		// Early exit if the smallest number is greater than 0
		if nums[i] > 0 {
			break;
		}

		// Skip duplicate numbers
		if i > 0 && nums[i] == nums[i - 1] {
			continue;
		}

		let mut left_i = i + 1;
		let mut right_i = nums_len - 1;

		while left_i < right_i {
			match nums[i] + nums[left_i] + nums[right_i] {
				sum if sum < 0 => {
					left_i = get_next_i(Side::Left, left_i, right_i)
				}

				sum if sum > 0 => {
					right_i = get_next_i(Side::Right, left_i, right_i)
				}

				_ => {
					triplets.push(vec![nums[i], nums[left_i], nums[right_i]]);
					left_i = get_next_i(Side::Left, left_i, right_i);
					right_i = get_next_i(Side::Right, left_i, right_i);
				}
			}
		}
	}

	triplets
}

#[cfg(test)]
mod tests {
	use super::three_sum;

	#[test]
	fn test_example_1() {
		let nums = vec![-1, 0, 1, 2, -1, -4];
		let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
		let mut result = three_sum(nums);
		result.iter_mut().for_each(|v| v.sort());
		result.sort();
		assert_eq!(result, expected);
	}

	#[test]
	fn test_example_2() {
		let nums = vec![0, 0, 0];
		let expected = vec![vec![0, 0, 0]];
		let mut result = three_sum(nums);
		result.iter_mut().for_each(|v| v.sort());
		result.sort();
		assert_eq!(result, expected);
	}

	#[test]
	fn test_no_solution() {
		let nums = vec![1, 2, -1, -2];
		let expected: Vec<Vec<i32>> = vec![];
		let result = three_sum(nums);
		assert_eq!(result, expected);
	}

	#[test]
	fn test_large_numbers() {
		let nums = vec![1000, 2000, 5000, -6000];
		let expected = vec![vec![-6000, 1000, 5000]];
		let mut result = three_sum(nums);
		result.iter_mut().for_each(|v| v.sort());
		result.sort();
		assert_eq!(result, expected);
	}

	#[test]
	fn test_mixed_numbers() {
		let nums = vec![-1, 2, 1, -4, 0, 1];
		let expected = vec![vec![-1, 0, 1]];
		let mut result = three_sum(nums);
		result.iter_mut().for_each(|v| v.sort());
		result.sort();
		assert_eq!(result, expected);
	}
}
