use super::coordinate::*;
use std::cmp::{max, min, Ordering};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Rectangle {
	pub bottom_left: Coordinate,
	pub top_right: Coordinate,
	pub area: i32,
}

impl PartialOrd for Rectangle {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.area.cmp(&other.area))
	}
}

impl Ord for Rectangle {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl Rectangle {
	pub fn new(bottom_left: (i32, i32), top_right: (i32, i32)) -> Rectangle {
		let bottom_left = Coordinate::new(
			bottom_left.0,
			bottom_left.1,
			ComparisonMode::XFirst,
		);

		let top_right =
			Coordinate::new(top_right.0, top_right.1, ComparisonMode::YFirst);

		let area = Self::get_area(&bottom_left, &top_right);

		Rectangle {
			area,
			bottom_left,
			top_right,
		}
	}

	fn get_area(bottom_left: &Coordinate, top_right: &Coordinate) -> i32 {
		let width = top_right.x.abs_diff(bottom_left.x);
		let height = top_right.y.abs_diff(bottom_left.y);

		(width * height) as i32
	}

	pub fn get_overlapping_area(&self, rectangle_b: &Rectangle) -> i32 {
		let leftmost_right_x = min(self.top_right.x, rectangle_b.top_right.x);

		let rightmost_left_x =
			max(self.bottom_left.x, rectangle_b.bottom_left.x);

		let topmost_bottom_y =
			max(self.bottom_left.y, rectangle_b.bottom_left.y);

		let bottommost_top_y = min(self.top_right.y, rectangle_b.top_right.y);

		let overlap_width = leftmost_right_x - rightmost_left_x;
		let overlap_height = bottommost_top_y - topmost_bottom_y;

		match (overlap_width.is_negative(), overlap_height.is_negative()) {
			(false, false) => overlap_width * overlap_height,
			_ => 0,
		}
	}
}
