use rand::{rngs::ThreadRng, Rng, seq::SliceRandom};

use super::{grid::Grid, cell::{Cell, Direction}};

pub trait Ruleset {
	fn delete_rule(&mut self, rng: &mut ThreadRng);
	fn add_rule(&mut self, rng: &mut ThreadRng);
	fn expand_rule(&mut self, rng: &mut ThreadRng);
	fn contract_rule(&mut self, rng: &mut ThreadRng);
	fn separate_rule(&mut self, rng: &mut ThreadRng);
	fn mutate_cells(&mut self, rng: &mut ThreadRng);
	fn clear_dead_rules(&mut self);
	fn contract_empty_borders(&mut self);
}

impl Ruleset for Vec<Grid> {
    fn delete_rule(&mut self, rng: &mut ThreadRng) {
		let to_delete = rng.gen_range(1..self.len());

		self.remove(to_delete);
		
		let rules_len = self.len();
		for rule in self {
			for cell in rule.contents_mut() {
				if let Cell::Stem(n, _) = cell {
					if *n as usize > to_delete { *n -= 1 }
					if *n as usize == to_delete { *n = rng.gen_range(1..rules_len as u8) }	
				}
			}
		}
    }

    fn add_rule(&mut self, rng: &mut ThreadRng) {
		let rules_len = self.len();
		for rule in self.iter_mut() {
			for cell in rule.contents_mut() {
				if let Cell::Stem(n, _) = cell {
					if rng.gen_bool(1.0 / (rules_len + 1) as f64) {
						*n = rules_len as u8;
					}
				}
			}
		}

		self.push(Grid::random(rng, (self.len() + 1) as u8));
    }

    fn expand_rule(&mut self, rng: &mut ThreadRng) {
		let rules_len = self.len();
		let rule = self.choose_mut(rng).unwrap();
		rule.expand(Direction::random(rng), rng, rules_len as u8);
    }

    fn contract_rule(&mut self, rng: &mut ThreadRng) {
		let rule = self.choose_mut(rng).unwrap();
		rule.contract(Direction::random(rng));
    }

    fn separate_rule(&mut self, rng: &mut ThreadRng) {
		let choice = rng.gen_range(0..self.len());

		let mut stem_count = 0;
		for rule in self.iter() {
			for cell in rule.contents() {
				if let Cell::Stem(n, _) = cell {
					if *n == choice as u8 {
						stem_count += 1;
					}
				}
			}
		}

		if stem_count > 1 {
			stem_count = rng.gen_range(0..stem_count);
		
			let new_index = self.len() as u8;
			self.push(self[choice].clone());

			'outer: 
			for rule in self {
				for cell in rule.contents_mut() {
					if let Cell::Stem(n, _) = cell {
						if *n == choice as u8 {
							if stem_count == 0 {
								*n = new_index;

								break 'outer;
							}

							stem_count -= 1;
						}
					}
				}
			}
		}
	}

    fn mutate_cells(&mut self, rng: &mut ThreadRng) {
		let rules_len = self.len();
		for rule in self.iter_mut() {
			for cell in rule.contents_mut() {
				if rng.gen_bool(0.1) {
					*cell = Cell::random(rng, rules_len as u8);
				}
			}
		}
	}

    fn clear_dead_rules(&mut self) {
		let mut used = vec![false; self.len()];
		used[0] = true;
		for rule in self.iter() {
			for cell in rule.contents() {
				if let Cell::Stem(n, _) = cell {
					used[*n as usize] = true;
				}
			}
		}

		for (i, keep) in used.into_iter().enumerate().rev() {
			if keep { continue; }
			
			self.remove(i);
				
			for rule in self.iter_mut() {
				for cell in rule.contents_mut() {
					if let Cell::Stem(n, _) = cell {
						if *n as usize >= i { *n -= 1 }
					}
				}
			}
		}
	}

    fn contract_empty_borders(&mut self) {
		for rule in self.iter_mut() {
			rule.contract_empty();
		}
	}
}