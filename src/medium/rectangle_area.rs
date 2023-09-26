use crate::util::rectangle::Rectangle;

/// Difficulty: [Medium](https://leetcode.com/problems/rectangle-area/description/)
/// 
/// Computes the combined area of two rectangles given their bottom-left and
/// top-right coordinates.
///
/// The function determines the individual areas of both rectangles and subtracts
/// the overlapping area to prevent double counting.
///
/// # Arguments
///
/// * `ax1`, `ay1` - Bottom-left coordinates of the first rectangle.
/// * `ax2`, `ay2` - Top-right coordinates of the first rectangle.
/// * `bx1`, `by1` - Bottom-left coordinates of the second rectangle.
/// * `bx2`, `by2` - Top-right coordinates of the second rectangle.
///
/// # Examples
///
/// ```
/// assert_eq!(compute_area(0, 0, 2, 2, 1, 1, 3, 3), 7);
/// // The area of the first rectangle is 4, the area of the second rectangle
/// // is 4, and their overlapping area is 1. Hence, the combined area is 7.
/// ```
///
/// # Note
///
/// * This function assumes the existence of a `Rectangle` struct and its
///   associated methods (`new` and `get_overlapping_area`). Ensure these are
///   implemented for the function to work.
pub fn compute_area(
	ax1: i32,
	ay1: i32,
	ax2: i32,
	ay2: i32,
	bx1: i32,
	by1: i32,
	bx2: i32,
	by2: i32,
) -> i32 {
	let rectangle_a = Rectangle::new((ax1, ay1), (ax2, ay2));

	let rectangle_b = Rectangle::new((bx1, by1), (bx2, by2));

	let overlap = rectangle_a.get_overlapping_area(&rectangle_b);

	rectangle_a.area + rectangle_b.area - overlap
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_no_overlap() {
		assert_eq!(compute_area(0, 0, 2, 2, 3, 3, 5, 5), 8);
	}

	#[test]
	fn test_full_overlap() {
		assert_eq!(compute_area(0, 0, 2, 2, 0, 0, 2, 2), 4);
	}

	#[test]
	fn test_partial_overlap() {
		assert_eq!(compute_area(0, 0, 3, 3, 1, 1, 4, 4), 14);
	}

	#[test]
	fn test_edge_overlap() {
		assert_eq!(compute_area(0, 0, 3, 3, 3, 0, 5, 3), 15);
	}
}
