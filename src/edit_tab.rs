use egui_macroquad::{macroquad::prelude::*, egui::{Context, DragValue, SidePanel, panel::Side, Vec2, vec2, Button, ScrollArea}};
use soft_evolution::l_system::{grid::Grid, cell::{Cell, Direction}};

use crate::{controls::Controls, drawing::{draw_grid_lines, draw_grid, pixel_width}, state::Tab, ui::{centered_button, rule_button}};

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
	send: Option<(usize, Vec<Grid>)>
}

impl EditTab {

	pub fn rules_ui(&mut self, ctx: &Context) {
		let mut to_delete = None;

		SidePanel::new(Side::Left, "Rule Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				ScrollArea::vertical().show(ui, |ui| {
					for (i, rule) in self.l_rules.iter().enumerate() {
						let (big_resp, small_resp) = rule_button(ui, rule, i, self.l_rules.len(), self.current_rule == i);
						if big_resp.clicked() {
							self.current_rule = i;
						}
						if small_resp.clicked() {
							to_delete = Some(i);
						}
					}
	
					ui.add_space(7.5);
					
					if centered_button(ui, Vec2::new(150.0, 25.0), "\u{2795}").clicked() {
						self.l_rules.push(Grid::single(Cell::Passive));
					}

					ui.add_space(160.0);
				});
			});
		
		if let Some(i) = to_delete {
			if self.current_rule >= i {
				self.current_rule -= 1;
			}
			self.l_rules.remove(i);
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
						ui.horizontal(|ui| {
							let tmp = ui.spacing().item_spacing;
							ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
							if ui.add_enabled(self.draw_stem_type < 255, Button::new("\u{2795}")).clicked() { 
								self.draw_stem_type += 1;
							}
							if ui.add_enabled(self.draw_stem_type > 0, Button::new("\u{2796}")).clicked() { 
								self.draw_stem_type -= 1;
							}
							ui.spacing_mut().item_spacing = tmp;
						});
						ui.horizontal(|ui| {
							ui.add(DragValue::new(&mut self.draw_stem_type).speed(0.1).clamp_range(0..=255));
							ui.label("Stem cell type");
						});
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
				if centered_button(ui, vec2(150.0, 25.0), "Send to Grow").clicked() {
					self.send = Some((2, self.l_rules.clone()));
				}
			});
    }
}

impl Tab for EditTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
			l_rules: vec![
				Grid::new(3, 3, vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Stem(0, Direction::UP), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty], [1, 1]),
				Grid::new(3, 3, vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Stem(0, Direction::UP), Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty], [1, 1]),
			],
			tool: EditTool::Draw,
			draw_stem_type: 0,
			draw_stem_dir: Direction::UP,
			draw_cell: CellType::Passive,
			current_rule: 0,
			send: None,
		}
    }

    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update(can_use_mouse);

		if is_mouse_button_down(MouseButton::Left) && can_use_mouse {
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
		
		draw_grid_lines(&self.l_rules[self.current_rule], pixel_width(self.controls.camera()));
		draw_grid(&self.l_rules[self.current_rule]);    
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