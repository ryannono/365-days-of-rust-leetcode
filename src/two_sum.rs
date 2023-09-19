use std::collections::HashMap;

/// Finds two numbers within `nums` that sum up to `target` and returns their
/// indices. If no such pair is found, returns an empty vector.
///
/// This function iterates over `nums` and uses a hash map to keep track of each
/// number's index. For each number, it calculates the complementary number that,
/// when added to the current number, will equal the `target`. If this complement
/// is found in the hash map, it means a pair of numbers that sum up to `target`
/// has been found, and their indices are returned.
///
/// # Parameters
///
/// - `nums`: A vector of integers in which to search for the pair.
/// - `target`: The target sum we are trying to achieve with the pair of numbers.
///
/// # Returns
///
/// A vector containing two integers representing the indices of the pair of
/// numbers that add up to `target`. If no such pair is found, returns an empty
/// vector.
///
/// # Example
///
/// ```
/// use your_crate_name::two_sum;
///
/// let nums = vec![2, 7, 11, 15];
/// let target = 9;
///
/// let result = two_sum(nums, target);
/// assert_eq!(result, vec![1, 0]);
/// ```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut num_to_index: HashMap<i32, usize> = HashMap::new();

	for (index, &num) in nums.iter().enumerate() {
		if let Some(&hashed_complement_index) =
			num_to_index.get(&(target - num))
		{
			return vec![index as i32, hashed_complement_index as i32];
		} else {
			num_to_index.insert(num, index);
		}
	}

	vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert!(result.contains(&0) && result.contains(&1));
    }

    #[test]
    fn test_two_sum_2() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert!((result.contains(&1) && result.contains(&2)) || (result.contains(&0) && result.contains(&1)));
    }

    #[test]
    fn test_two_sum_3() {
        let result = two_sum(vec![3, 3], 6);
        assert!(result.contains(&0) && result.contains(&1));
    }
}