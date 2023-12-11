use egui_macroquad::{macroquad::prelude::*, egui::{Context, DragValue, SidePanel, panel::Side, Vec2, vec2, Button, ScrollArea, Color32, Layout, Align}};
use soft_evolution::l_system::{grid::Grid, cell::{Cell, Direction}, is_valid};

use crate::{controls::Controls, drawing::{draw_grid_lines, draw_grid, pixel_width, draw_grid_axes}, state::Tab, ui::{centered_button, rule_button, RuleButtonResponse}};

#[derive(PartialEq)]
enum EditTool {
	Draw, Erase // Rotate
}

#[derive(PartialEq, Eq)]
enum CellType {
	Stem,
	Passive,
}

pub struct EditTab {
	controls: Controls,
	current_rule: usize,
	l_rules: Vec<Grid>,
	tool: EditTool,
	draw_cell: CellType,
	draw_stem_type: u8,
	draw_stem_dir: Direction,
	send: Option<(usize, Vec<Grid>)>,
	send_error: bool,
}

impl EditTab {

	pub fn rules_ui(&mut self, ctx: &Context) {
		use RuleButtonResponse as Resp;
		let mut resp = (Resp::None, 0);

		SidePanel::new(Side::Left, "Rule Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ScrollArea::vertical().show(ui, |ui| {
					for (i, rule) in self.l_rules.iter().enumerate() {
						let this_resp = rule_button(ui, rule, i, self.l_rules.len(), self.current_rule == i);
						if this_resp != Resp::None {
							resp = (this_resp, i);
						}
					}
	
					ui.add_space(7.5);
					
					if centered_button(ui, Vec2::new(150.0, 25.0), "\u{2795}").clicked() {
						self.l_rules.push(Grid::single(Cell::Passive));
						self.current_rule = self.l_rules.len() - 1;
					}

					ui.add_space(160.0);
				});
			});
		

		match resp {
			(Resp::None, _) => {},
			(Resp::Select, i) => self.current_rule = i,
			(Resp::Delete, i) => {
				if self.current_rule >= i && self.current_rule != 0 {
					self.current_rule -= 1;
				}
				self.l_rules.remove(i);
			},
			(Resp::MoveUp, i) => {
				self.l_rules.swap(i, i-1);
				if i == self.current_rule {
					self.current_rule -= 1;
				}
				else if i - 1 == self.current_rule {
					self.current_rule += 1;
				}
			},
			(Resp::MoveDown, i) => {
				self.l_rules.swap(i, i+1);
				if i == self.current_rule {
					self.current_rule += 1;
				}
				else if i + 1 == self.current_rule {
					self.current_rule -= 1;
				}
			},
			(Resp::Duplicate, i) => {
				self.l_rules.insert(i, self.l_rules[i].clone());
				if self.current_rule > i {
					self.current_rule += 1;
				}
			},
		}
	}

	fn tools_ui(&mut self, ctx: &Context) {
		SidePanel::new(Side::Right, "Tool Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.selectable_value(&mut self.tool, EditTool::Draw, "Draw");
					ui.selectable_value(&mut self.tool, EditTool::Erase, "Erase");
				});

				ui.separator();
				
				ui.add_visible_ui(self.tool == EditTool::Draw, |ui| {
					ui.radio_value(&mut self.draw_cell, CellType::Passive, "Passive");
					ui.radio_value(&mut self.draw_cell, CellType::Stem, "Stem");

					ui.separator();
					
					ui.add_visible_ui(self.draw_cell == CellType::Stem, |ui| {
						ui.allocate_ui_with_layout(vec2(0.0, 0.0), Layout::left_to_right(Align::Center), |ui| {
							ui.vertical(|ui| {
								ui.horizontal(|ui| {
									let tmp = ui.spacing().item_spacing;
									ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
									if ui.add_enabled(self.draw_stem_type > 0, Button::new("\u{2796}")).clicked() { 
										self.draw_stem_type -= 1;
									}
									if ui.add_enabled(self.draw_stem_type < 255, Button::new("\u{2795}")).clicked() { 
										self.draw_stem_type += 1;
									}
									ui.spacing_mut().item_spacing = tmp;
								});
								ui.horizontal(|ui| {
									ui.add(DragValue::new(&mut self.draw_stem_type).speed(0.1).clamp_range(0..=255));
								});
							});
							ui.label("Stem cell type");
						});

						ui.add_space(10.0);

						ui.radio_value(&mut self.draw_stem_dir, Direction::UP, "Up");
						ui.radio_value(&mut self.draw_stem_dir, Direction::RIGHT, "Right");
						ui.radio_value(&mut self.draw_stem_dir, Direction::DOWN, "Down");
						ui.radio_value(&mut self.draw_stem_dir, Direction::LEFT, "Left");
						
						ui.separator();
					});
				});

				ui.separator();
				if centered_button(ui, vec2(150.0, 25.0), "Optimize").clicked() {
					self.l_rules[self.current_rule].contract_empty();
				}

				ui.separator();
				if centered_button(ui, vec2(150.0, 25.0), "Send to Grow").clicked() {
					if is_valid(&self.l_rules) {
						self.send = Some((2, self.l_rules.clone()));
					}
					else {
						self.send_error = true;
					}
				}

				if self.send_error {
					ui.colored_label(Color32::RED, "Cannot send, invalid stem cells.");
				}
			});
    }
}

impl Tab for EditTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
			l_rules: vec![
				Grid::vertical(vec![Cell::Stem(0, Direction::RIGHT), Cell::Passive, Cell::Passive], 0)
			],
			tool: EditTool::Draw,
			draw_stem_type: 0,
			draw_stem_dir: Direction::UP,
			draw_cell: CellType::Passive,
			current_rule: 0,
			send: None,
			send_error: false,
		}
    }

    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update(can_use_mouse);

		if is_mouse_button_down(MouseButton::Left) && can_use_mouse {
			self.send_error = false;

			let pos: [i32; 2] = self.controls.mouse_world.floor().as_ivec2().into();
			let pos = [pos[0] as isize, -pos[1] as isize];
			
			let rule = &mut self.l_rules[self.current_rule];
			
			rule.insert_cell(match self.tool {
				EditTool::Draw => match self.draw_cell {
					CellType::Stem => Cell::Stem(self.draw_stem_type, self.draw_stem_dir),
					CellType::Passive => Cell::Passive,
				},
				EditTool::Erase => Cell::Empty,
			}, pos);
		}
		
        set_camera(self.controls.camera());
		
		let pixel = pixel_width(self.controls.camera());
		draw_grid_lines(&self.l_rules[self.current_rule], pixel);
		draw_grid(&self.l_rules[self.current_rule]);
		draw_grid_axes(&self.l_rules[self.current_rule], pixel);
	}

	fn draw_ui(&mut self, ctx: &Context) {
		self.rules_ui(ctx);
		self.tools_ui(ctx);
	}

	fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
		if let Some((i, grid)) = self.send.take() {
			self.send = None;

			return Some((i, grid));
		}
		None
	}

	fn receive(&mut self, _system: Vec<Grid>) {
		
	}
}