use egui_macroquad::{macroquad::prelude::*, egui::{Context, DragValue, SidePanel, panel::Side, Vec2, vec2, Button, ScrollArea, Color32, Layout, Align, Window}};
use soft_evolution::l_system::{grid::Grid, cell::{Cell, Direction}, is_valid};

use crate::{controls::Controls, drawing::{draw_grid_lines, draw_grid, pixel_width, draw_grid_axes}, state::Tab, ui::{centered_button, rule_button, RuleButtonResponse}, files::{is_alphanumeric, save_rules, load_rules, get_filenames}};

#[derive(PartialEq)]
enum EditTool {
	Draw, Erase
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
	send: Option<usize>,
	send_error: bool,

	saving_window: bool,
	save_filename: String,
	save_disclaimer: Option<String>,

	loading_window: bool,
	load_filenames: Vec<String>,
	load_disclaimer: Option<String>,
	load_selected: usize,
}

impl EditTab {

	pub fn rules_ui(&mut self, ctx: &Context) {
		use RuleButtonResponse as Resp;
		let mut resp = (Resp::None, 0);

		SidePanel::new(Side::Left, "Rule Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ui.set_enabled(!self.saving_window && !self.loading_window);
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
				ui.set_enabled(!self.saving_window && !self.loading_window);

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
								ui.add(DragValue::new(&mut self.draw_stem_type).speed(0.1).clamp_range(0..=255));
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
				ui.label("Rule options");
				if centered_button(ui, vec2(150.0, 25.0), "Optimize").clicked() {
					self.l_rules[self.current_rule].contract_empty();
				}
				if centered_button(ui, vec2(150.0, 25.0), "Clear").clicked() {
					self.l_rules[self.current_rule].clear();
				}
				if centered_button(ui, vec2(150.0, 25.0), "Rotate Left").clicked() {
					self.l_rules[self.current_rule].rotate(Direction::LEFT);
				}
				if centered_button(ui, vec2(150.0, 25.0), "Rotate Right").clicked() {
					self.l_rules[self.current_rule].rotate(Direction::RIGHT);
				}
				if centered_button(ui, vec2(150.0, 25.0), "Send to Evolve").clicked() {
					self.send = Some(1);
				}

				ui.separator();
				ui.label("LSystem options");
				if centered_button(ui, vec2(150.0, 25.0), "Save").clicked() {
					self.saving_window = true;
				}
				if centered_button(ui, vec2(150.0, 25.0), "Load").clicked() {
					self.loading_window = true;
					self.load_filenames = get_filenames();
					self.load_selected = 0;
				}
				if centered_button(ui, vec2(150.0, 25.0), "Send to Grow").clicked() {
					if is_valid(&self.l_rules) {
						self.send = Some(2);
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

	fn draw_saving_window(&mut self, ctx: &Context) {
		Window::new("Save")
			.collapsible(false)
			.constraint_to(ctx.screen_rect())
			.show(ctx, |ui| {

				ui.text_edit_singleline(&mut self.save_filename);

				ui.colored_label(Color32::RED, self.save_disclaimer.as_ref().unwrap_or(&"".into()));

				if ui.button("save").clicked() {
					if self.save_filename.trim().len() == 0 {
						self.save_disclaimer = Some("Empty filename".into());
					}
					else if !is_alphanumeric(&self.save_filename) {
						self.save_disclaimer = Some("Non-alphanumeric characters found".into());
					}
					else {
						if let Err(e) = save_rules(&self.l_rules, &self.save_filename) {
							self.save_disclaimer = Some(e.to_string());
						}
						else {
							self.saving_window = false;
							self.load_disclaimer = None;
						}
					}
				}
				if ui.button("cancel").clicked() {
					self.saving_window = false;
					self.load_disclaimer = None;
				}
			});
	}

	fn draw_loading_window(&mut self, ctx: &Context) {
		Window::new("Load")
			.collapsible(false)
			.constraint_to(ctx.screen_rect())
			.show(ctx, |ui| {

				ui.label("Choose the file to load:");

				ScrollArea::vertical()
					.max_height(200.0)
					.auto_shrink([false, true])
					.show(ui, |ui| {
						for (i, name) in self.load_filenames.iter().enumerate() {
							if ui.selectable_label(self.load_selected == i, name).clicked() {
								self.load_selected = i;
							}
						}
					});

				ui.colored_label(Color32::RED, self.load_disclaimer.as_ref().unwrap_or(&"".into()));

				if ui.button("load").clicked() {
					match load_rules(&self.load_filenames[self.load_selected]) {
						Ok(rules) => {
							self.receive(rules);
							self.loading_window = false;
							self.load_disclaimer = None;
						},
						Err(message) => self.load_disclaimer = Some(message),
					}
				}
				if ui.button("cancel").clicked() {
					self.loading_window = false;
					self.load_disclaimer = None;
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
			saving_window: false,
			save_filename: String::new(),
			save_disclaimer: None,
			loading_window: false,
			load_selected: 0,
			load_filenames: Vec::new(),
			load_disclaimer: None,
		}
    }

    fn frame(&mut self, mut can_use_mouse: bool) {
		can_use_mouse &= !self.saving_window && !self.loading_window;
		self.controls.update(can_use_mouse);

		if is_mouse_button_down(MouseButton::Left) && can_use_mouse {
			self.send_error = false;

			let pos: [i32; 2] = self.controls.mouse_world.floor().as_ivec2().into();
			let pos = [pos[0], -pos[1]];
			
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

		if self.saving_window || self.loading_window {
			set_camera(&Camera2D::default());
			draw_rectangle(-1.0, -1.0, 2.0, 2.0, Color::from_rgba(30, 30, 30, 100));
		}
	}

	fn draw_ui(&mut self, ctx: &Context) {
		self.rules_ui(ctx);
		self.tools_ui(ctx);

		if self.saving_window {
			self.draw_saving_window(ctx);
		}
		if self.loading_window {
			self.draw_loading_window(ctx);
		}
	}

	fn send_to(&mut self) -> Option<(usize, Vec<Grid>)> {
		if let Some(i) = self.send.take() {
			if i == 2 {
				return Some((i, self.l_rules.clone()));
			}
			else {
				return Some((i, self.l_rules[self.current_rule..=self.current_rule].into()))
			}
		}
		None
	}

	fn receive(&mut self, system: Vec<Grid>) {
		self.l_rules = system;
		self.current_rule = 0;
	}
}