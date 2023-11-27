use rand::rngs::ThreadRng;

pub trait Evolve {
	fn new_random(rng: &mut ThreadRng) -> Self;
	fn new_mutated(other: &Self, factor: f32, rng: &mut ThreadRng) -> Self;
	fn fitness(&self) -> f32;
}