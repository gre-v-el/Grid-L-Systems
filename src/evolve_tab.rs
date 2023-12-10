use egui_macroquad::{macroquad::prelude::*, egui::Context};
use soft_evolution::{genetic_algorithm::GeneticAlgorithm, l_system::{grid::Grid, cell::Cell}};

use crate::{controls::Controls, state::Tab, ls_evolve::LS};

pub struct EvolveTab {
	controls: Controls,
	gen_alg: GeneticAlgorithm<LS, Grid>,
}

impl Tab for EvolveTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
			gen_alg: GeneticAlgorithm::new(100, 30, 1.0, Grid::from_string(include_str!("templates/cross.txt"), [2, 2]).unwrap()),
		}
    }
	
    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update(can_use_mouse);
		set_camera(self.controls.camera());
		
		
    }

    fn draw_ui(&mut self, ctx: &Context) {
        
    }

    fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
        None
    }

    fn receive(&mut self, _system: Vec<Grid>) {
        
    }
}