use egui_macroquad::{macroquad::prelude::*, egui::{Context, Window}};

use crate::{controls::Controls, state::Tab};

pub struct EvolveTab {
	controls: Controls,
}

impl Tab for EvolveTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
		}
    }
	
    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update(can_use_mouse);
		set_camera(self.controls.camera());
        draw_rectangle(0.0, 0.0, 1.0, 1.0, RED);
    }

    fn draw_ui(&mut self, ctx: &Context) {
        Window::new("EvolveInspector")
			.title_bar(false)
			.collapsible(false)
			.fixed_pos((10.0, 35.0))
			.movable(false)
			.resizable(false)
			.show(ctx, |ui| {
				ui.label("Evolve");
			}
		);
    }

    fn send_to(&mut self) -> Option<(usize, Vec<soft_evolution::l_system::grid::Grid>)> {
        None
    }

    fn receive(&mut self, _system: Vec<soft_evolution::l_system::grid::Grid>) {
        
    }
}