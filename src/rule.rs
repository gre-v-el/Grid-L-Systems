use crate::grid::Grid;

pub struct Rule {
	pub from: u8,
	pub to: Grid
}

impl Rule {
	pub fn new(from: u8, to: Grid) -> Self {
		Self {
			from,
			to,
		}
	}
}