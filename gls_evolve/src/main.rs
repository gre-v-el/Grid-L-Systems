mod ls_evolve;
mod controls;
mod state;
mod edit_tab;
mod evolve_tab;
mod grow_tab;
mod drawing;
mod ui;
mod files;

use std::env;
use egui_macroquad::macroquad::{self, prelude::*};

use state::State;

#[macroquad::main("GLS Evolver")]
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