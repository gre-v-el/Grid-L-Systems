use egui_macroquad::{macroquad::prelude::*, egui::{Context, SidePanel, panel::Side, vec2, Slider}};
use soft_evolution::l_system::{grid::Grid, LSystem, cell::{Direction, Cell}};

use crate::{controls::Controls, state::Tab, drawing::{draw_grid_lines, pixel_width, draw_grid_axes, draw_grid_animated, draw_grid}, ui::centered_button};

pub struct GrowTab {
	controls: Controls,
	
	system: LSystem,
	prev_system: Grid,
	animated_rule: usize,
	animated_from: Option<([i32; 2], Direction)>,

	running: bool,
	iteration: u32,
	step_delay: f64,
	last_update: f64,
	
	show_grid: bool,
	animate: bool,

	send: Option<usize>,
}

impl GrowTab {
	fn step_system(&mut self) {
		self.last_update = get_time();
		self.prev_system = self.system.state().clone();
		if let Some(pos) = self.system.queue().front() {
			let cell = self.system.state().at(*pos);
			if let Cell::Stem(n, dir) = cell {
				self.animated_from = Some((*pos, dir));
				self.animated_rule = n as usize;
			}
			else {
				panic!();
			}
			self.iteration += 1;
			self.system.try_step();
		}
		else {
			self.animated_from = None;
		}

	}
}

impl Tab for GrowTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),

			system: LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), vec![
				Grid::single(Cell::Stem(0, Direction::UP))
			]),
			prev_system: Grid::single(Cell::Empty),
			animated_rule: 0,
			animated_from: None,

			running: false,
			iteration: 0,
			step_delay: 1.0,
			last_update: -1.0,

			show_grid: false,
			animate: true,

			send: None,
		}
    }

    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update(can_use_mouse);
		set_camera(self.controls.camera());

		let pixel = pixel_width(self.controls.camera());
		if self.show_grid {
			draw_grid_lines(self.system.state(), pixel);
		}

		
		if !self.animate {
			draw_grid(self.system.state());
		}
		else if let Some((pos, dir)) = self.animated_from {
			let mut t = (get_time() - self.last_update) as f32 / self.step_delay as f32;
			if t > 1.0 { 
				t = 1.0;
				self.animated_from = None;
			}
			draw_grid_animated(self.system.state(), &self.prev_system, &self.system.rules()[self.animated_rule], pos, dir, t);
			
		}
		else {
			draw_grid(self.system.state());
		}


		if self.show_grid {
			draw_grid_axes(self.system.state(), pixel);
		}

		if self.running && get_time() - self.step_delay > self.last_update {
			self.step_system();
		}
	}

    fn draw_ui(&mut self, ctx: &Context) {
		SidePanel::new(Side::Right, "Tool Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ui.label(format!("steps: {}", self.iteration));

				let text = if self.running { "Pause" } else { "Grow" };
				if centered_button(ui, vec2(150.0, 25.0), text).clicked() {
					self.running = !self.running;
				}
				
				ui.add_enabled_ui(!self.running, |ui| {
					if centered_button(ui, vec2(150.0, 25.0), "Step").clicked() {
						self.step_system();		
					}
					if centered_button(ui, vec2(150.0, 25.0), "Reset").clicked() {
						self.system.set_state(Grid::single(Cell::Stem(0, Direction::UP)));
						self.iteration = 0;
					}
				});

				ui.separator();

				let mut val = 1.0 - (self.step_delay * 0.5).powf(0.2);

				ui.label("Speed");
				ui.add(Slider::new(&mut val, 0.0..=1.0).show_value(false));

				self.step_delay = (1.0 - val).powf(5.0) * 2.0;
			
				ui.separator();

				ui.checkbox(&mut self.show_grid, "Show Grid");
				ui.checkbox(&mut self.animate, "Animate");

				ui.separator();

				if centered_button(ui, vec2(150.0, 25.0), "Send to Edit").clicked() {
					self.send = Some(0);
				}
				if centered_button(ui, vec2(150.0, 25.0), "Send to Evolve").clicked() {
					self.send = Some(1);
				}
			}
		);
    }

    fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
        if let Some(i) = self.send.take() {
			if i == 1 {
				return Some((i, vec![self.system.state().clone()]));
			}
			else {
				return Some((i, self.system.rules().into()));
			}
		}
		
		None
    }

    fn receive(&mut self, system: Vec<Grid>) {
        self.system = LSystem::new(Grid::single(Cell::Stem(0, Direction::UP)), system);
		self.running = false;
		self.iteration = 0;
    }
	
	
}