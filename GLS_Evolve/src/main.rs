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
save/load
smooth killing
fitness optimizations
fitness -> loss
mutation factor
fitness components weights
fitness - discriminate cell details
*/