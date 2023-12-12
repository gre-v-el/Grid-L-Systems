use std::time::Instant;

use egui_macroquad::{macroquad::prelude::*, egui::{Context, SidePanel, panel::Side, vec2, Sense, CentralPanel, DragValue, Rect, Stroke, Color32}};
use soft_evolution::{genetic_algorithm::GeneticAlgorithm, l_system::grid::Grid};

use crate::{state::Tab, ls_evolve::LS, ui::{draw_grid_ui, centered_button}};

fn number_suffix(n: usize) -> &'static str {
	match (n) % 10 {
		1 => "st",
		2 => "nd",
		3 => "rd",
		_ => "th",
	}
}

pub struct EvolveTab {
	running: bool,
	gen_alg: GeneticAlgorithm<LS, Grid>,
	showing: usize,
	evolve_budget: u16,
	selected: usize,
}

impl Tab for EvolveTab {
    fn new() -> Self {
		let goal = Grid::from_string(include_str!("templates/cross.txt"), [2, 2]).unwrap();

        Self {
			gen_alg: GeneticAlgorithm::<LS, Grid>::new(100, 30, 1.0, goal),
			running: false,
			showing: 16,
			evolve_budget: 10,
			selected: 0,
		}
    }
	
    fn frame(&mut self, _: bool) {
		if self.running {
			let start = Instant::now();
			while start.elapsed().as_millis() < self.evolve_budget as u128 {
				self.gen_alg.perform_generation();
			}
		}
    }

    fn draw_ui(&mut self, ctx: &Context) {
        SidePanel::new(Side::Right, "Evolution Inspector")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ui.label("target:");

				let (target_rect, _) = ui.allocate_exact_size(vec2(140.0, 100.0), Sense::hover());
				draw_grid_ui(ui, &self.gen_alg.goal(), target_rect);

				ui.separator();
				
				ui.label(format!("Generation: {}", self.gen_alg.generation_number()));

				if centered_button(ui, vec2(150.0, 25.0), if self.running { "Pause" } else { "Evolve" }).clicked() {
					self.running = !self.running;
				}

				ui.add_enabled_ui(!self.running, |ui| {
					if centered_button(ui, vec2(150.0, 25.0), "Step").clicked() {
						self.gen_alg.perform_generation();
					}
				});

				ui.separator();

				ui.horizontal(|ui|{
					ui.add(DragValue::new(&mut self.showing).clamp_range(1..=self.gen_alg.agents().len()).speed(0.05));
					ui.label("Show");
				});
				ui.horizontal(|ui|{
					ui.add(DragValue::new(&mut self.evolve_budget).speed(0.2));
					ui.label("Evolve Budget");
				});
			});

			
		SidePanel::new(Side::Left, "Agent Inspector")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				let inspected = &self.gen_alg.agents()[self.selected];
				let (agent_rect, _) = ui.allocate_exact_size(vec2(140.0, 100.0), Sense::hover());
				draw_grid_ui(ui, inspected.0.0.state(), agent_rect);
				
				ui.separator();

				ui.label(format!("{}{} out of {}", self.selected + 1, number_suffix(self.selected + 1), self.gen_alg.agents().len()));
				ui.label(format!("fitness: {}", inspected.1));
			});

			CentralPanel::default().show(ctx, |ui| {
				let origin = ui.next_widget_position();
				let available = ui.available_size();
				let aspect = available.x / available.y;
				
				let rows = (self.showing as f32 / aspect).sqrt();
				let cols = aspect * rows;

				let mut rows = rows.round() as usize;
				let mut cols = cols.round() as usize;

				while cols * rows < self.showing {
					if cols > rows {
						cols += 1;
					}
					else {
						rows += 1;
					}
				}

				let size = vec2(
					available.x / cols as f32,
					available.y / rows as f32,
				);

				for i in 0..self.showing {
					let row = i / cols;
					let col = i % cols;

					let pos = origin + vec2(col as f32, row as f32) * size;

					
					let rect = Rect::from_min_size(pos, size).expand(-5.0);

					let resp = ui.allocate_rect(rect, Sense::click());
					
					let background = if resp.hovered() { Color32::from_gray(50) } else { Color32::TRANSPARENT };
					ui.painter().rect(rect, 0.0, background, Stroke::new(2.0, Color32::DARK_GRAY));

					let agent_index = (i as f32 / (self.showing - 1) as f32 * (self.gen_alg.agents().len() - 1) as f32) as usize;
					draw_grid_ui(ui, self.gen_alg.agents()[agent_index].0.0.state(), rect.expand(-5.0));

					if resp.clicked() {
						self.selected = agent_index;
					}
					resp.on_hover_ui_at_pointer(|ui| {
						ui.label(format!("{}{}", agent_index + 1, number_suffix(agent_index + 1)));
					});
				}
			});
    }

    fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
        None
    }

    fn receive(&mut self, _system: Vec<Grid>) {
        
    }
}