use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
	Stem(u8, Direction),
	Passive,
	Empty,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
	UP, LEFT, DOWN, RIGHT
}

impl Direction {
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