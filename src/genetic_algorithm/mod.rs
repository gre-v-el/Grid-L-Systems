mod evolve;

use evolve::Evolve;

use rand::{seq::SliceRandom, rngs::ThreadRng, thread_rng};

pub struct GeneticAlgorithm<T: Evolve> {
	rng: ThreadRng,
	mutation_factor: f32,
	generation_count: usize,
	survivors_count: usize,
	agents: Vec<(T, f32)>,
	generation_number: usize,
}

impl<T: Evolve> GeneticAlgorithm<T> {
	pub fn new(generation_count: usize, survivors_count: usize, mutation_factor: f32) -> Self {
		let mut rng = thread_rng();
		let agents = (0..generation_count).map(|_| (T::new_random(&mut rng), 0.0)).collect();
		let mut ret = Self {
			mutation_factor,
			rng,
			generation_count,
			survivors_count,
			agents,
			generation_number: 0,
		};

		ret.calculate_fitnesses();

		ret
	}

	fn calculate_fitnesses(&mut self) {
		for (agent, fitness) in &mut self.agents {
			*fitness = agent.fitness();
		}
	}

	fn reproduce(&mut self) {
		while self.agents.len() < self.generation_count {
			let parent = &self.agents.choose(&mut self.rng).unwrap().0;
			let new = T::new_mutated(parent, self.mutation_factor, &mut self.rng);
			self.agents.push((new, 0.0));
		}
	}

	pub fn perform_generation(&mut self) {
		self.agents.truncate(self.survivors_count);
		self.reproduce();
		self.calculate_fitnesses();
		self.agents.sort_unstable_by(|e1, e2| e2.1.total_cmp(&e1.1));

		self.generation_number += 1;
	}

	pub fn perform_generations(&mut self, n: usize) {
		for _ in 0..n {
			self.perform_generation();
		}
	}

	pub fn best(&self) -> &(T, f32) {
		self.agents.first().unwrap()
	}

	pub fn worst(&self) -> &(T, f32) {
		self.agents.last().unwrap()
	}

	pub fn median(&self) -> &(T, f32) {
		&self.agents[self.generation_count / 2]
	}
} 