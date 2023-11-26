use std::collections::VecDeque;

use crate::{grid::Grid, rule::Rule, cell::Cell};

pub struct LSystem<const N: usize> {
	state: Grid,
	rules: [Rule; N],
	stem_queue: VecDeque<[isize; 2]>,
}

impl<const N: usize> LSystem<N> {
	pub fn new(state: Grid, rules: [Rule; N]) -> Self {
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

	pub fn try_step(&mut self) {
		if self.stem_queue.is_empty() { return; }

		let pos = self.stem_queue.pop_front().unwrap();

		if let Cell::Stem(stem_type, stem_dir) = self.state.at(pos) {
			let mut matched = None;

			for rule in &self.rules {
				if rule.from == stem_type {
					matched = Some(&rule.to);
					break;
				}
			}

			if let Some(to) = matched {
				self.state.insert(to, pos, stem_dir);

				self.stem_queue.retain(|e| {
					!to.contains(self.state.pos_to_other_pos(*e, pos))
				});

				for (other_pos, cell) in to {
					match cell {
						Cell::Stem(_, _) => {
							let other_pos = stem_dir.rotate_coords(other_pos[0], other_pos[1]);
							let state_pos = to.pos_to_other_pos(other_pos, [-pos[0], -pos[1]]);
							self.stem_queue.push_back(state_pos);
						}
						_ => {}
					}
				}

			}
		}
		else {
			panic!();
		}
	}

	pub fn state(&self) -> &Grid {
		&self.state
	}
}