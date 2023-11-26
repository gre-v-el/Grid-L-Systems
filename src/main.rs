mod rule;
mod grid;
mod l_system;
mod cell;

use cell::Cell::*;
use grid::Grid;
use l_system::LSystem;
use rule::Rule;


fn main() {
    let mut system = LSystem::new(
		Grid::single(Stem(0)),
		[
			Rule::new(0, Grid::new(3, 3, &[
					Passive, Passive, Stem(1),
					Passive, Empty,   Empty,
					Stem(2), Empty,   Empty,
				], 
				[0, 0]
			)),
			Rule::new(1, Grid::vertical  (&[Stem(3), Passive, Passive], 2)),
			Rule::new(2, Grid::horizontal(&[Stem(4), Passive, Passive], 2)),
			Rule::new(3, Grid::horizontal(&[Passive, Passive, Stem(1)], 0)),
			Rule::new(4, Grid::vertical  (&[Passive, Passive, Stem(2)], 0)),
		],
	);

	for _ in 0..34 {
		println!("{}\n", system.state());
		system.try_step();
	}
	println!("{}", system.state());
}
