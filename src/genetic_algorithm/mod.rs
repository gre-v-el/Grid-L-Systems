pub mod evolve;

use evolve::Evolve;

use rand::{seq::SliceRandom, rngs::ThreadRng, thread_rng, Rng};

pub struct GeneticAlgorithm<T, U> where T: Evolve<U> {
	rng: ThreadRng,
	pub mutation_factor: f32,
	pub generation_count: usize,
	pub survivors_count: usize,
	pub tournament_size: usize,
	agents: Vec<(T, f32)>,
	generation_number: u32,
	params: U,
}

impl<T, U> GeneticAlgorithm<T, U> where T: Evolve<U> {
	pub fn new(generation_count: usize, survivors_count: usize, mutation_factor: f32, params: U) -> Self {
		let mut rng = thread_rng();
		let agents = (0..generation_count).map(|_| (T::new_random(&mut rng), 0.0)).collect();
		let mut ret = Self {
			mutation_factor,
			rng,
			generation_count,
			survivors_count,
			agents,
			generation_number: 0,
			params,
			tournament_size: 5,
		};

		ret.calculate_fitnesses();

		ret
	}

	fn calculate_fitnesses(&mut self) {
		for (agent, fitness) in &mut self.agents {
			*fitness = agent.fitness(&self.params);
		}
	}

	fn reproduce(&mut self) {
		while self.agents.len() < self.generation_count {
			let parent = &self.agents.choose(&mut self.rng).unwrap().0;
			let new = T::new_mutated(parent, self.mutation_factor, &mut self.rng);
			self.agents.push((new, 0.0));
		}
	}

	fn select(&mut self) {
		// in-place tournament selection

		let mut tournament = Vec::with_capacity(self.tournament_size);
		for i in 1..self.survivors_count {
			tournament.clear();
			for _ in 0..self.tournament_size {
				tournament.push(self.rng.gen_range(i..self.agents.len()));
			}

			let mut max_index = 0;
			let mut max_fitness = -f32::MAX;
			for j in 0..self.tournament_size {
				let contender = &self.agents[tournament[j]];
				if contender.1 > max_fitness {
					max_index = j;
					max_fitness = contender.1;
				}
			}

			self.agents.swap(i, tournament[max_index]);
		}
		
		self.agents.truncate(self.survivors_count);
	}

	pub fn perform_generation(&mut self) {
		self.select();
		self.agents.iter_mut().for_each(|e| e.0.reset());
		self.reproduce();
		self.calculate_fitnesses();
		self.agents.sort_unstable_by(|e1, e2| e2.1.total_cmp(&e1.1));

		self.generation_number += 1;
	}

	pub fn reset(&mut self) {
		self.generation_number = 0;
		for agent in &mut self.agents {
			*agent = (T::new_random(&mut self.rng), 0.0);
		}
		self.calculate_fitnesses();
	}

	pub fn perform_generations(&mut self, n: u32) {
		for _ in 0..n {
			self.perform_generation();
		}
	}

	pub fn set_params(&mut self, params: U) {
		self.params = params;
	}

	pub fn params(&self) -> &U {
		&self.params
	}

	pub fn params_mut(&mut self) -> &mut U {
		&mut self.params
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

	pub fn agents(&self) -> &[(T, f32)] {
		&self.agents
	}

	pub fn generation_number(&self) -> u32 {
		self.generation_number
	}
} 