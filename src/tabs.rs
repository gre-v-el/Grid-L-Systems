use egui_macroquad::{macroquad::prelude::*, egui::{Context, Window}};
use soft_evolution::l_system::{LSystem, grid::Grid, cell::Cell};

use crate::controls::Controls;

pub trait Tab {
	fn new() -> Self where Self: Sized;
	fn update(&mut self);
	fn set_camera(&self);
	fn draw_scene(&mut self);
	fn draw_ui(&mut self, ctx: &Context);
}

pub struct EditTab {
	controls: Controls,
	l_rules: Vec<Grid>,
}

impl Tab for EditTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
			l_rules: vec![
				Grid::new(3, 3, vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Passive, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty], [1, 1]),
			],
		}
    }

    fn update(&mut self) {
		self.controls.update();
    }

    fn set_camera(&self) {
        set_camera(self.controls.camera());
    }

    fn draw_scene(&mut self) {
        draw_rectangle(0.0, 0.0, 1.0, 1.0, RED);
    }

    fn draw_ui(&mut self, ctx: &Context) {
        Window::new("EditInspector")
			.title_bar(false)
			.collapsible(false)
			.fixed_pos((10.0, 35.0))
			.movable(false)
			.resizable(false)
			.show(ctx, |ui| {
				ui.label("Edit");
			}
		);
    }
}


pub struct EvolveTab {
	controls: Controls,
}

impl Tab for EvolveTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
		}
    }

    fn update(&mut self) {
		self.controls.update();
    }

    fn set_camera(&self) {
        set_camera(self.controls.camera());
    }

    fn draw_scene(&mut self) {
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
}

pub struct GrowTab {
	controls: Controls,
}

impl Tab for GrowTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
		}
    }

    fn update(&mut self) {
		self.controls.update();
    }

    fn set_camera(&self) {
        set_camera(self.controls.camera());
    }

    fn draw_scene(&mut self) {
        draw_rectangle(0.0, 0.0, 1.0, 1.0, RED);
    }

    fn draw_ui(&mut self, ctx: &Context) {
        Window::new("GrowInspector")
			.title_bar(false)
			.collapsible(false)
			.fixed_pos((10.0, 35.0))
			.movable(false)
			.resizable(false)
			.show(ctx, |ui| {
				ui.label("Grow");
			}
		);
    }
}