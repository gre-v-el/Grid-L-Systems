pub mod cell;
pub mod grid;
pub mod ruleset;

use std::collections::VecDeque;
use grid::Grid;
use cell::Cell;

#[derive(Clone)]
pub struct LSystem {
	state: Grid,
	rules: Vec<Grid>,
	stem_queue: VecDeque<[i32; 2]>,
}

impl LSystem {
	pub fn new(state: Grid, rules: Vec<Grid>) -> Self {
		assert!(rules.len() > 0);
		
		let mut stem_queue = VecDeque::new();
		
		for ([x, y], cell) in &state {
			match cell {
				Cell::Stem(_, _) => stem_queue.push_back([x, y]),
				_ => {}
			}
		}
		
		Self {
			state,
			rules,
			stem_queue,
		}
	}

	pub fn try_step(&mut self) -> bool {
		if self.stem_queue.is_empty() { return false; }

		let pos = self.stem_queue.pop_front().unwrap();

		if let Cell::Stem(stem_type, stem_dir) = self.state.at(pos) {
			let to = &self.rules[stem_type as usize];

			self.state.insert(to, pos, stem_dir);

			self.stem_queue.retain(|e| {
				match self.state.at(*e) {
					Cell::Stem(_, _) => true,
					_ => false,
				}
			});

			for (other_pos, cell) in to {
				match cell {
					Cell::Stem(_, _) => {
						let other_pos = stem_dir.rotate_coords([other_pos[0], other_pos[1]]);
						let state_pos = to.pos_to_other_pos(other_pos, [-pos[0], -pos[1]]);
						self.stem_queue.push_back(state_pos);
					}
					_ => {}
				}
			}

			// if the cell that was just used didn't cover itself up, push it back
			if let Cell::Stem(_, _) = self.state.at(pos) {
				self.stem_queue.push_back(pos);
			}
		}
		else {
			panic!();
		}

		return true;
	}

	pub fn state(&self) -> &Grid {
		&self.state
	}

	pub fn set_state(&mut self, grid: Grid) {
		self.state = grid;

		self.stem_queue.clear();

		for ([x, y], cell) in &self.state {
			match cell {
				Cell::Stem(_, _) => self.stem_queue.push_back([x, y]),
				_ => {}
			}
		}
	}

	pub fn rules(&self) -> &[Grid] {
		&self.rules
	}

	pub fn queue(&self) -> &VecDeque<[i32; 2]> {
		&self.stem_queue
	}
}

pub fn is_valid(rules: &[Grid]) -> bool {
	let max_stem = (rules.len() - 1) as u8;

	for rule in rules {
		for (_, cell) in rule {
			if let Cell::Stem(n, _) = cell {
				if n > max_stem {
					return false;
				}
			}
		}
	}

	true
}