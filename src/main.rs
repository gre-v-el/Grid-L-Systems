mod rule;
mod grid;
mod l_system;
mod cell;

use std::env;

use cell::Cell::*;
use grid::Grid;
use l_system::LSystem;
use rule::Rule;

use crate::cell::Direction;


fn main() {
	env::set_var("RUST_BACKTRACE", "1");

    let mut system = LSystem::new(
		Grid::single(Stem(0, Direction::UP)),
		[
			Rule::new(0, Grid::new(3, 3, &[
					Passive, Passive, Stem(1, Direction::DOWN),
					Passive, Empty, Empty,
					Stem(2, Direction::LEFT), Empty, Empty,
				],
				[0, 0]
			)),
			Rule::new(1, Grid::vertical  (&[Stem(2, Direction::RIGHT), Passive, Passive], 2)),
			Rule::new(2, Grid::vertical  (&[Stem(1, Direction::LEFT), Passive, Passive], 2)),
		],
	);

	for _ in 0..34 {
		println!("{}\n", system.state());
		system.try_step();
	}
	println!("{}", system.state());
}
