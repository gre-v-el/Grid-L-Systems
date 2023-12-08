use std::{fmt::Display, ops::Neg};

use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
	Stem(u8, Direction),
	Passive,
	Empty,
}

impl Cell {
	pub fn random(rng: &mut ThreadRng, stem_types: u8) -> Self {
		match rng.gen_range(0..8) {
			0 		=> Cell::Stem(rng.gen_range(1..stem_types), Direction::random(rng)),
			1..=4 	=> Cell::Passive,
			_ 		=> Cell::Empty,
		}
	}

	pub fn same_type(&self, other: &Self) -> bool {
		match self {
			Cell::Stem(_, _) => match other {
				Cell::Stem(_, _) => true,
				Cell::Passive => 	false,
				Cell::Empty => 		false,
			},
			Cell::Passive => 	match other {
				Cell::Stem(_, _) => false,
				Cell::Passive => 	true,
				Cell::Empty => 		false,
			},
			Cell::Empty => 		match other {
				Cell::Stem(_, _) => false,
				Cell::Passive => 	false,
				Cell::Empty => 		true,
			},
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
	UP, LEFT, DOWN, RIGHT
}

impl Direction {

	pub fn random(rng: &mut ThreadRng) -> Self {
		match rng.gen_range(0..4) {
			0 => Direction::UP,
			1 => Direction::LEFT,
			2 => Direction::DOWN,
			_ => Direction::RIGHT,
		}
	}

	pub fn rotate_vals<T: Copy>(&self, a: T, b: T, c: T, d: T) -> [T; 4] {
		match self {
			Direction::UP => [a, b, c, d],
			Direction::LEFT => [d, a, b, c],
			Direction::DOWN => [c, d, a, b],
			Direction::RIGHT => [b, c, d, a],
		}
	}
	
	pub fn rotate_coords<T>(&self, v: [T; 2]) -> [T; 2]
	where T: Copy + Neg<Output = T>
	{
		match self {
			Direction::UP => [v[0], v[1]],
			Direction::LEFT => [-v[1], v[0]],
			Direction::DOWN => [-v[0], -v[1]],
			Direction::RIGHT => [v[1], -v[0]],
		}
	}

	pub fn rotate_cell(&self, cell: Cell) -> Cell {
		match cell {
			Cell::Stem(n, d) => Cell::Stem(n, self.rotate_dir(d)),
			c => c
		}
	}

	pub fn rotate_dir(&self, dir: Direction) -> Direction {
		match self {
			Direction::UP => 	dir,
			Direction::LEFT => 	match dir {
				Direction::UP => 	Direction::LEFT,
				Direction::LEFT => 	Direction::DOWN,
				Direction::DOWN => 	Direction::RIGHT,
				Direction::RIGHT => Direction::UP,
			},
			Direction::DOWN => 	match dir {
				Direction::UP =>	Direction::DOWN,
				Direction::LEFT => 	Direction::RIGHT,
				Direction::DOWN => 	Direction::UP,
				Direction::RIGHT => Direction::LEFT,
			},
			Direction::RIGHT => match dir {
				Direction::UP => 	Direction::RIGHT,
				Direction::LEFT => 	Direction::UP,
				Direction::DOWN => 	Direction::LEFT,
				Direction::RIGHT => Direction::DOWN,
			},
		}
	}
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::UP => write!(f, "^"),
            Direction::LEFT => write!(f, "<"),
            Direction::DOWN => write!(f, "v"),
            Direction::RIGHT => write!(f, ">"),
        }
    }
}