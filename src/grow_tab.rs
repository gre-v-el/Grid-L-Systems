use egui_macroquad::{macroquad::prelude::*, egui::{Context, SidePanel, panel::Side, vec2, Slider}};
use soft_evolution::l_system::{grid::Grid, LSystem, cell::{Direction, Cell}};

use crate::{controls::Controls, state::Tab, drawing::{draw_grid_lines, pixel_width, draw_grid}, ui::centered_button};

pub struct GrowTab {
	controls: Controls,
	system: LSystem,
	running: bool,
	step_delay: f64,
	last_update: f64,
}

impl Tab for GrowTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
			system: LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), vec![
				Grid::vertical(vec![Cell::Passive, Cell::Passive, Cell::Stem(0, Direction::RIGHT)], 2)
			]),
			running: false,
			step_delay: 1.0,
			last_update: -1.0,
		}
    }

    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update(can_use_mouse);
		set_camera(self.controls.camera());


		draw_grid_lines(self.system.state(), pixel_width(self.controls.camera()));
		draw_grid(self.system.state());

		if self.running && get_time() - self.step_delay > self.last_update {
			self.last_update = get_time();
			self.system.try_step();
		}
	}

    fn draw_ui(&mut self, ctx: &Context) {
		SidePanel::new(Side::Right, "Tool Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ui.add_enabled_ui(!self.running, |ui| {
					if centered_button(ui, vec2(150.0, 25.0), "Step").clicked() {
						self.system.try_step();
					}
				});

				let text = if self.running { "Pause" } else { "Grow" };
				
				if centered_button(ui, vec2(150.0, 25.0), text).clicked() {
					self.running = !self.running;
				}

				ui.separator();

				let mut val = 1.0 - (self.step_delay * 0.5).powf(0.2);

				ui.label("Speed");
				ui.add(Slider::new(&mut val, 0.0..=1.0).show_value(false));

				self.step_delay = (1.0 - val).powf(5.0) * 2.0;
			}
		);
    }

    fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
        None
    }

    fn receive(&mut self, system: Vec<Grid>) {
        self.system = LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), system);
		self.running = false;
    }
	
	
}