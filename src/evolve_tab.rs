use std::{thread, sync::mpsc::{channel, Sender, TryRecvError, Receiver}};

use egui_macroquad::{macroquad::prelude::*, egui::{Context, SidePanel, panel::Side, vec2, Sense}};
use soft_evolution::{genetic_algorithm::GeneticAlgorithm, l_system::grid::Grid};

use crate::{controls::Controls, state::Tab, ls_evolve::LS, ui::{draw_grid_ui, centered_button}, drawing::draw_grid};

enum ThreadInstruction {
	OneGeneration,
	Start,
	Stop,
	RequestBest,
}

enum EvolutionView {
	Best(Grid, usize),
}

pub struct EvolveTab {
	controls: Controls,
	instruction_sender: Sender<ThreadInstruction>,
	view_receiver: Receiver<EvolutionView>,
	goal: Grid,
	requested: bool,
	view: Option<EvolutionView>,
	running: bool,
}

impl EvolveTab {
	fn generation(&self) -> usize {
		match &self.view {
			None => 0,
			Some(view) => match view {
				EvolutionView::Best(_, g) => *g,
			}
		}
	}
}

impl Tab for EvolveTab {
    fn new() -> Self {
		let goal = Grid::from_string(include_str!("templates/cross.txt"), [2, 2]).unwrap();
		let goal_clone = goal.clone();
		let (instruction_sender, instruction_receiver) = channel();
		let (view_sender, view_receiver) = channel();

		thread::spawn(move || {
			let mut gen_alg = GeneticAlgorithm::<LS, Grid>::new(100, 30, 1.0, goal_clone);

			let mut running = false;

			loop {
				let received = instruction_receiver.try_recv(); 
				
				match received {
					Ok(ThreadInstruction::RequestBest) => {
						let best = gen_alg.best().0.0.state().clone();
						let view = EvolutionView::Best(best, gen_alg.generation_number());
						view_sender.send(view).unwrap()
					},
					Ok(ThreadInstruction::OneGeneration) => gen_alg.perform_generation(),
					Ok(ThreadInstruction::Start) => running = true,
					Ok(ThreadInstruction::Stop) => running = false,
					Err(TryRecvError::Disconnected) => panic!(),
					Err(TryRecvError::Empty) => {
						if running {
							gen_alg.perform_generation();
						}
					}
				}
			};
		});

		instruction_sender.send(ThreadInstruction::RequestBest).unwrap();

        Self {
			controls: Controls::new(),
			instruction_sender,
			goal,
			view: None,
			view_receiver,
			running: false,
			requested: true,
		}
    }
	
    fn frame(&mut self, can_use_mouse: bool) {
		if self.running && !self.requested {
			self.instruction_sender.send(ThreadInstruction::RequestBest).unwrap();
			self.requested = true;
		}
		if let Ok(view) = self.view_receiver.try_recv() {
			self.view = Some(view);
			self.requested = false;
		}

		self.controls.update(can_use_mouse);
		set_camera(self.controls.camera());
		

		if let Some(view) = &self.view {
			match view {
				EvolutionView::Best(best, _) => {
					draw_grid(best);
				},
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
				draw_grid_ui(ui, &self.goal, target_rect);

				ui.separator();

				if centered_button(ui, vec2(150.0, 25.0), "One Generation").clicked() {
					self.instruction_sender.send(ThreadInstruction::OneGeneration).unwrap();
					self.instruction_sender.send(ThreadInstruction::RequestBest).unwrap();
					self.requested = true;
				}
				
				if centered_button(ui, vec2(150.0, 25.0), if self.running { "Pause" } else { "Evolve" }).clicked() {
					self.instruction_sender.send(if self.running { ThreadInstruction::Stop } else { ThreadInstruction::Start }).unwrap();

					self.running = !self.running;
				}

			});

			
		SidePanel::new(Side::Left, "Evolution Data")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				
				ui.label(format!("Generation: {}", self.generation()));
			});
    }

    fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
        None
    }

    fn receive(&mut self, _system: Vec<Grid>) {
        
    }
}