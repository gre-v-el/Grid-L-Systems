use rand::Rng;
use rand::rngs::ThreadRng;

use soft_evolution::l_system::LSystem;
use soft_evolution::genetic_algorithm::evolve::Evolve;
use soft_evolution::l_system::cell::{Cell, Direction};
use soft_evolution::l_system::grid::Grid;
use soft_evolution::l_system::ruleset::Ruleset;

use crate::evolve_tab::EvolveParams;


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

impl Evolve<EvolveParams> for LS {
    fn new_random(rng: &mut ThreadRng) -> Self {
		let stem_types = rng.gen_range(1..=5u8);
		let mut rules = Vec::with_capacity(stem_types as usize);

		for _ in 0..stem_types {
			rules.push(Grid::random(rng, stem_types));
		}

        Self(LSystem::new(Grid::single(Cell::Stem(rng.gen_range(0..stem_types), Direction::UP)), rules))
    }

    fn reset(&mut self) {
        self.0.set_state(Grid::single(Cell::Stem(0, Direction::UP)));
    }

    fn new_mutated(other: &Self, factor: f32, rng: &mut ThreadRng) -> Self {
		let mut rules = Vec::from(other.0.rules());

		let choice = rng.gen_range(0.0..=20.0 - 14.8 * factor);
		match choice as usize {
			0 if rules.len() > 2 => rules.delete_rule(rng),
			1 => rules.add_rule(rng),
			2 => rules.expand_rule(rng),
			3 => rules.contract_rule(rng),
			4 => rules.separate_rule(rng),
			_ => rules.mutate_cells(rng, factor as f64 * 0.5 + 0.01),
		}

		rules.clear_dead_rules();
		rules.contract_empty_borders();

		LS(LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), rules))
    }

    fn fitness(&mut self, params: &EvolveParams) -> f32 {
    	for _ in 0..params.max_steps {
		   if !self.0.try_step() { break; }
		}

		let (same, different) = params.goal.score_simmilarity(self.0.state());
		let mut size = 0.0;

		for rule in self.0.rules() {
			size += (rule.contents().len() as f32).powf(params.size_pow);
		}

		same as f32 * params.same_weight + different as f32 * params.different_weight + size * params.size_weight
	}
}