mod grid;
mod l_system;
mod cell;

use std::env;

use cell::Cell::*;
use grid::Grid;
use l_system::LSystem;

use crate::cell::Direction;


fn main() {
	env::set_var("RUST_BACKTRACE", "1");

    let mut system = LSystem::new(
		Grid::single(Stem(0, Direction::UP)),
		[
			Grid::new(3, 3, &[
					Passive, Passive, Stem(1, Direction::DOWN),
					Passive, Empty, Empty,
					Stem(2, Direction::LEFT), Empty, Empty,
				],
				[0, 0]
			),
			Grid::vertical(&[Passive, Passive, Stem(2, Direction::LEFT)], 0),
			Grid::vertical(&[Passive, Passive, Stem(1, Direction::RIGHT)], 0),
		],
	);

	for _ in 0..34 {
		println!("{}\n", system.state());
		system.try_step();
	}
	println!("{}", system.state());
}
