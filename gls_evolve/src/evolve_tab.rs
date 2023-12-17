use std::time::Instant;

use egui_macroquad::{macroquad::prelude::*, egui::{Context, SidePanel, panel::Side, vec2, Sense, CentralPanel, Rect, Stroke, Color32}};
use soft_evolution::{genetic_algorithm::GeneticAlgorithm, l_system::grid::Grid};

use crate::{state::Tab, ls_evolve::LS, ui::{draw_grid_ui, centered_button, drag_label}};

fn number_suffix(n: usize) -> &'static str {
	match (n) % 10 {
		1 => "st",
		2 => "nd",
		3 => "rd",
		_ => "th",
	}
}

pub struct EvolveParams {
	pub goal: Grid,
	pub max_steps: u16,

	pub same_weight: f32,
	pub different_weight: f32,
	pub size_weight: f32,
	pub size_pow: f32,
}

pub struct EvolveTab {
	running: bool,
	gen_alg: GeneticAlgorithm<LS, EvolveParams>,
	visible_count: usize,
	visible_up_to: usize,
	evolve_budget: u16,
	selected: usize,

	send_selected: Option<usize>,
	send_target: bool,
}

impl Tab for EvolveTab {
    fn new() -> Self {
		let goal = Grid::from_string(include_str!("templates/cross.txt"), [2, 2]).unwrap();

		let params = EvolveParams {
			goal,
			max_steps: 25,

			same_weight: 1.0,
			different_weight: -10.0,
			size_weight: -0.5,
			size_pow: 1.5,
		};

        Self {
			gen_alg: GeneticAlgorithm::<LS, EvolveParams>::new(1000, 500, 0.5, params),
			running: false,
			visible_count: 16,
			visible_up_to: 1000,
			evolve_budget: 50,
			selected: 0,
			send_selected: None,
			send_target: false,
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
				ui.label(format!("Generation: {}", self.gen_alg.generation_number()));

				if centered_button(ui, vec2(150.0, 25.0), if self.running { "Pause" } else { "Evolve" }).clicked() {
					self.running = !self.running;
				}

				ui.add_enabled_ui(!self.running, |ui| {
					if centered_button(ui, vec2(150.0, 25.0), "Step").clicked() {
						self.gen_alg.perform_generation();
					}
					if centered_button(ui, vec2(150.0, 25.0), "Reset").clicked() {
						self.gen_alg.reset();
					}
				});
				

				ui.separator();
				ui.label("GA settings");

				drag_label(ui, &mut self.gen_alg.generation_count, 2..=10000, 5.0, "Generation Count");
				drag_label(ui, &mut self.gen_alg.survivors_count, 1..=(self.gen_alg.generation_count-1), 1.0, "Survivors Count");
				drag_label(ui, &mut self.gen_alg.tournament_size, 2..=(self.gen_alg.generation_count-1), 0.05, "Tournament Size");
				drag_label(ui, &mut self.gen_alg.mutation_factor, 0.0..=1.0, 0.002, "Mutation Factor");
				
				ui.add_space(5.0);
				ui.label("Fitness settings:");
				drag_label(ui, &mut self.gen_alg.params_mut().same_weight, 0.0..=10.0, 0.01, "Equal Cells");
				drag_label(ui, &mut self.gen_alg.params_mut().different_weight, -10.0..=0.0, 0.01, "Different Cells");
				drag_label(ui, &mut self.gen_alg.params_mut().size_pow, 0.0..=10.0, 0.01, "Size Power");
				drag_label(ui, &mut self.gen_alg.params_mut().size_weight, -10.0..=0.0, 0.01, "Size");
				drag_label(ui, &mut self.gen_alg.params_mut().max_steps, 1..=500, 0.04, "Max Steps");

				ui.separator();

				drag_label(ui, &mut self.visible_count, 1..=self.visible_up_to, 0.3, "Visible");
				drag_label(ui, &mut self.visible_up_to, self.visible_count..=self.gen_alg.agents().len(), 0.3, "Visible Range");
				drag_label(ui, &mut self.evolve_budget, 1..=1000, 0.2, "Evolve Budget");
			});

			
		SidePanel::new(Side::Left, "Agent Inspector")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {

				ui.label("target:");

				let (target_rect, _) = ui.allocate_exact_size(vec2(140.0, 100.0), Sense::hover());
				draw_grid_ui(ui, &self.gen_alg.params().goal, target_rect);

				if centered_button(ui, vec2(150.0, 25.0), "Send to Edit").clicked() {
					self.send_target = true;
				}

				ui.separator();
				ui.separator();

				ui.label("selected:");

				let inspected = &self.gen_alg.agents()[self.selected];
				let (agent_rect, _) = ui.allocate_exact_size(vec2(140.0, 100.0), Sense::hover());
				draw_grid_ui(ui, inspected.0.0.state(), agent_rect);
				
				ui.separator();

				ui.label(format!("{}{} out of {}", self.selected + 1, number_suffix(self.selected + 1), self.gen_alg.agents().len()));
				ui.label(format!("fitness: {:.2}", inspected.1));

				ui.separator();

				if centered_button(ui, vec2(150.0, 25.0), "Send to Edit").clicked() {
					self.send_selected = Some(0);
				}
				if centered_button(ui, vec2(150.0, 25.0), "Send to Grow").clicked() {
					self.send_selected = Some(2);
				}
			});

			CentralPanel::default().show(ctx, |ui| {
				let origin = ui.next_widget_position();
				let available = ui.available_size();
				let aspect = available.x / available.y;
				
				let rows = (self.visible_count as f32 / aspect).sqrt();
				let cols = aspect * rows;

				let mut rows = rows.round() as usize;
				let mut cols = cols.round() as usize;

				while cols * rows < self.visible_count {
					if cols > rows || cols == 0 {
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

				for i in 0..self.visible_count {
					let row = i / cols;
					let col = i % cols;

					let pos = origin + vec2(col as f32, row as f32) * size;

					
					let rect = Rect::from_min_size(pos, size).expand(-3.0);

					let resp = ui.allocate_rect(rect, Sense::click());

					let agent_index = (i as f32 / (self.visible_count - 1) as f32 * (self.visible_up_to-1).min(self.gen_alg.agents().len() - 1) as f32) as usize;
					
					let importance = if resp.hovered() {50} else {0} + if self.selected == agent_index {40} else {0};
					let background = Color32::from_gray(importance);
					ui.painter().rect(rect, 0.0, background, Stroke::new(2.0, Color32::DARK_GRAY));

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
		if self.send_target {
			self.send_target = false;
			return Some((0, vec![self.gen_alg.params().goal.clone()]));
		}

        if let Some(i) = self.send_selected.take() {
			return Some((i, self.gen_alg.agents()[self.selected].0.0.rules().into()));
		}
		None
    }

    fn receive(&mut self, system: Vec<Grid>) {
		let goal = system.into_iter().next().unwrap();
        self.gen_alg.params_mut().goal = goal;
    }
}