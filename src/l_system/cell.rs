use std::fmt::Display;

use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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
	
	pub fn rotate_coords(&self, x: isize, y: isize) -> [isize; 2] {
		match self {
			Direction::UP => [x, y],
			Direction::LEFT => [-y, x],
			Direction::DOWN => [-x, -y],
			Direction::RIGHT => [y, -x],
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