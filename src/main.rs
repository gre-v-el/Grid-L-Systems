mod l_system;

use std::env;

use l_system::grid::Grid;
use l_system::LSystem;
use l_system::cell::Direction;


fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	
	use l_system::cell::Cell as C;
    let mut system = LSystem::new(
		Grid::single(C::Stem(0, Direction::UP)),
		vec![
			Grid::new(5, 5, &[
				C::Empty,					C:: Empty,	C::Stem(1, Direction::DOWN),C::Empty, 	C::Empty,
				C::Empty,					C:: Empty,	C::Passive,					C::Empty, 	C::Empty,
				C::Stem(1, Direction::LEFT),C:: Passive,C::Passive, 			 	C::Passive, C::Stem(1, Direction::RIGHT),
				C::Empty,					C:: Empty,	C::Passive,					C::Empty, 	C::Empty,
				C::Empty,					C:: Empty,	C::Stem(1, Direction::UP), 	C::Empty, 	C::Empty,
			],
			[2, 2]),
			Grid::new(5, 3, &[
				C::Stem(1, Direction::LEFT), 	C:: Passive,	C::Passive, 			 C::Passive, C::Stem(1, Direction::RIGHT),
				C::Empty,   					C:: Empty,	C::Passive,					 C::Empty, C::Empty,
				C::Empty,   					C:: Empty,	C::Stem(1, Direction::UP), 	 C::Empty, C::Empty,
			],
			[2, 0]),
		],
	);

	for _ in 0..50 {
		println!("{}", system.state());
		system.try_step();
	}
	println!("{}", system.state());
}
