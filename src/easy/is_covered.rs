use std::collections::HashSet;

/**
Difficulty: [Easy](https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/)

Determines if all the integers in the range `left` to `right` (inclusive) are
covered by at least one of the ranges specified in the `ranges` vector.

This function creates a hash set containing all the integers covered by the
ranges in the `ranges` vector. Then, it checks if any integers in the range 
`left` to `right` (inclusive) are not present in this set.

# Parameters

- `ranges`: A vector of vectors where each inner vector contains a range 
  represented by two integers.
- `left`: The left bound of the range of integers to check.
- `right`: The right bound of the range of integers to check.

# Returns

A boolean value indicating whether all the integers in the range `left` to 
`right` (inclusive) are covered by at least one of the ranges specified in 
the `ranges` vector.

# Example

use your_crate_name::is_covered;

let ranges = vec![vec![1, 10], vec![10, 20], vec![1, 6]];
let left = 14;
let right = 15;

let result = is_covered(ranges, left, right);
assert_eq!(result, true);
*/
pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
	let ints: HashSet<i32> = ranges
		.into_iter()
		.flat_map(|range| (range[0]..=range[1]))
		.collect();

	let unseen_ints: Vec<i32> =
		(left..=right).filter(|x| !ints.contains(x)).collect();

	unseen_ints.len() == 0
}

#[cfg(test)]
mod tests {
    use super::is_covered;

    #[test]
    fn test_covered() {
        assert!(is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 1, 6));
    }

    #[test]
    fn test_not_covered() {
        assert!(!is_covered(vec![vec![1, 2], vec![3, 4]], 1, 5));
    }

    #[test]
    fn test_partial_overlap() {
        assert!(is_covered(vec![vec![1, 4], vec![3, 5]], 2, 4));
    }

    #[test]
    fn test_single_range() {
        assert!(is_covered(vec![vec![1, 5]], 2, 4));
    }

    #[test]
    fn test_single_point_covered() {
        assert!(is_covered(vec![vec![1, 5]], 5, 5));
    }

    #[test]
    fn test_single_point_not_covered() {
        assert!(!is_covered(vec![vec![1, 5]], 6, 6));
    }

    #[test]
    fn test_wide_range_not_covered() {
        assert!(!is_covered(vec![vec![1, 2], vec![4, 5]], 1, 6));
    }
}
