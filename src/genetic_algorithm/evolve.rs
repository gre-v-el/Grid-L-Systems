use rand::rngs::ThreadRng;

pub trait Evolve<T> {
	fn new_random(rng: &mut ThreadRng) -> Self;
	fn reset(&mut self);
	fn new_mutated(other: &Self, factor: f32, rng: &mut ThreadRng) -> Self;
	fn fitness(&mut self, params: &T) -> f32;
}