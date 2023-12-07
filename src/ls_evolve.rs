use lazy_static::lazy_static;
use rand::Rng;
use rand::rngs::ThreadRng;

use rand::seq::SliceRandom;
use soft_evolution::l_system::LSystem;
use soft_evolution::genetic_algorithm::evolve::Evolve;
use soft_evolution::l_system::cell::{Cell, Direction};
use soft_evolution::l_system::grid::Grid;

lazy_static!(
	static ref TEMPLATE: Grid = Grid::from_string(include_str!("templates/cross-circle.txt"), [5, 5]).unwrap();
);


pub fn random_grid(rng: &mut ThreadRng, stem_types: u8) -> Grid {
	let width = rng.gen_range(1..=4);
	let height = rng.gen_range(1..=4);

	let mut contents = Vec::with_capacity(width * height);

	for _ in 0..(width*height) {
		let cell = Cell::random(rng, stem_types);
		contents.push(cell);
	}

	Grid::new(width, height, contents, [rng.gen_range(0..width), rng.gen_range(0..height)])
}

#[derive(Clone)]
pub struct LS(pub LSystem);

impl LS {
	#[allow(dead_code)]
	pub fn new(rules: Vec<Grid>) -> Self {
		Self(
			LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), rules)
		)
	}
}

impl Evolve for LS {
    fn new_random(rng: &mut ThreadRng) -> Self {
		let stem_types = rng.gen_range(2..=5u8);
		let mut rules = Vec::with_capacity(stem_types as usize);

		for _ in 0..stem_types {
			rules.push(random_grid(rng, stem_types));
		}

        Self(LSystem::new(Grid::single(Cell::Stem(rng.gen_range(0..stem_types), Direction::UP)), rules))
    }

    fn reset(&mut self) {
        self.0.set_state(Grid::single(Cell::Stem(0, Direction::UP)));
    }

    fn new_mutated(other: &Self, _factor: f32, rng: &mut ThreadRng) -> Self {
		let mut rules = Vec::from(other.0.rules());

		let choice = rng.gen_range(0..=10);
		match choice {
			// delete a rule
			0 if rules.len() > 2 => {
				let to_delete = rng.gen_range(1..rules.len());
				rules.remove(to_delete);
				
				let rules_len = rules.len();
				for rule in &mut rules {
					for cell in rule.contents_mut() {
						if let Cell::Stem(n, _) = cell {
							if *n as usize > to_delete { *n -= 1 }
							if *n as usize == to_delete{ *n = rng.gen_range(1..rules_len as u8) }	
						}
					}
				}
			},
			// add a rule
			1 => {
				let rules_len = rules.len();
				for rule in &mut rules {
					for cell in rule.contents_mut() {
						if let Cell::Stem(n, _) = cell {
							if rng.gen_bool(1.0 / (rules_len + 1) as f64) {
								*n = rules_len as u8;
							}
						}
					}
				}

				rules.push(random_grid(rng, (rules.len() + 1) as u8));
			},
			// expand a rule
			2 => {
				let rules_len = rules.len();
				let rule = rules.choose_mut(rng).unwrap();
				rule.expand(Direction::random(rng), rng, rules_len as u8);
			}
			// contract a rule
			3 => {
				let rule = rules.choose_mut(rng).unwrap();
				rule.contract(Direction::random(rng));
			}
			// mutate cells
			_ => {
				let rules_len = rules.len();
				for rule in &mut rules {
					for cell in rule.contents_mut() {
						if rng.gen_bool(0.1) {
							*cell = Cell::random(rng, rules_len as u8);
						}
					}
				}
			},
		}

		// // clear dead rules
		// let mut used = vec![false; rules.len()];
		// used[0] = true;
		// for rule in &rules {
		// 	for cell in rule.contents() {
		// 		if let Cell::Stem(n, _) = cell {
		// 			used[*n as usize] = true;
		// 		}
		// 	}
		// }

		// for (i, keep) in used.into_iter().enumerate().rev() {
		// 	if keep { continue; }
			
		// 	rules.remove(i);
				
		// 	for rule in &mut rules {
		// 		for cell in rule.contents_mut() {
		// 			if let Cell::Stem(n, _) = cell {
		// 				if *n as usize > i { *n -= 1 }
		// 			}
		// 		}
		// 	}
		// }

		// shrink empty borders
		for rule in &mut rules {
			rule.contract_empty();
		}

		LS(LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), rules))
    }

    fn fitness(&mut self) -> f32 {
    	for _ in 0..25 {
		   self.0.try_step();
		}

		let simmilarity = TEMPLATE.score_simmilarity(self.0.state());
		let mut rules_cells = 0;

		for rule in self.0.rules() {
			rules_cells += rule.contents().len() * rule.contents().len();
		}

		f32::powf(2.0, simmilarity) / rules_cells as f32
	}
}