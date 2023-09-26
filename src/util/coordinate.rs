use std::cmp::Ordering::{self, *};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Coordinate {
	pub x: i32,
	pub y: i32,
	comparison_mode: ComparisonMode,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ComparisonMode {
	XFirst, // x-first
	YFirst, // y-first
}

impl Coordinate {
	pub fn new(x: i32, y: i32, comparison_mode: ComparisonMode) -> Coordinate {
		Coordinate {
			x,
			y,
			comparison_mode,
		}
	}

	pub fn strict_cmp(&self, other: &Coordinate) -> Option<Ordering> {
		match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
			(Equal, Equal) => Some(Equal),
			(Greater, Greater) => Some(Greater),
			(Less, Less) => Some(Less),
			_ => None,
		}
	}
}

impl PartialOrd for Coordinate {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.comparison_mode {
			ComparisonMode::XFirst => {
				match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
					(Equal, y_ord) => Some(y_ord),
					(x_ord, _) => Some(x_ord),
				}
			}

			ComparisonMode::YFirst => {
				match (self.y.cmp(&other.y), self.x.cmp(&other.x)) {
					(Equal, x_ord) => Some(x_ord),
					(y_ord, _) => Some(y_ord),
				}
			}
		}
	}
}

impl Ord for Coordinate {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}
