mod ls_evolve;
mod controls;
mod state;
mod edit_tab;
mod evolve_tab;
mod grow_tab;
mod drawing;
mod ui;

use std::env;
use egui_macroquad::macroquad::{self, prelude::*};

use state::State;

#[macroquad::main("Grid L-systems")]
async fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	
	// let mut alg: GeneticAlgorithm<LS> = GeneticAlgorithm::new(150, 40, 1.0);
	// for i in 0..1000 {
	// 	if i % 100 == 0 { println!("gen {i}/1000"); }
	// 	alg.perform_generation();
	// }
	// let mut best = alg.best().0.clone();
	// best.reset();
	// let mut best = best.0;
	
	
	// for (i, rule) in best.rules().iter().enumerate() {
	// 	println!("{i}\nV\n{rule}");
	// }

	// println!("{}", "\n".repeat(5));
	
	// for _ in 0..25 {
	// 	println!("{}", best.state());
	// 	best.try_step();
	// }

	egui_macroquad::cfg(|ctx| {
		ctx.set_pixels_per_point(1.2);
	});

	let mut state= State::new();

	loop {
		clear_background(BLACK);

		state.frame();
		
		next_frame().await;
	}

}


/*
split the project into workspaces
save/load
smooth killing
fitness optimizations
fitness -> loss
mutation factor
fitness - discriminate cell details
change all usizes to u_ so that they can be serialized
*/