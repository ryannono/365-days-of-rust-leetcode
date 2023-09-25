use std::{collections::VecDeque, cmp::max};

pub mod easy;
pub mod hard;
pub mod medium;
pub mod util;

fn main() {
	println!("{}", total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]));
	println!("{}", total_steps(vec![4, 5, 7, 7, 13]));
	println!("{}", total_steps(vec![10, 1, 2, 3, 4, 5, 6, 1, 2, 3]));
	// println!("{}", count_primes(0));
	// println!("{}", count_primes(200));
	// println!("{}", count_primes(78675645));
}

pub fn total_steps(nums: Vec<i32>) -> i32 {
	let mut max_steps = 0;

	for i in 0..nums.len() {
		if let Some(next_greater_index) =
			(i + 1..nums.len()).find(|&j| nums[j] >= nums[i])
		{
			max_steps = max(max_steps, next_greater_index - i);
		}
	}

	(max_steps - 1) as i32
}
