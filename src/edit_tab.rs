use egui_macroquad::{macroquad::prelude::*, egui::{Context, DragValue, SidePanel, panel::Side, Vec2, Sense, Align2, FontId, Rect, vec2, pos2, Stroke}};
use soft_evolution::l_system::{grid::Grid, cell::{Cell, Direction}};

use crate::{controls::Controls, drawing::{draw_grid_lines, draw_grid, cell_col, arr_to_col}, state::Tab};

#[derive(PartialEq)]
enum EditTool {
	Draw, Erase // Rotate
}

pub struct EditTab {
	controls: Controls,
	current_rule: usize,
	l_rules: Vec<Grid>,
	tool: EditTool,
	draw_cell: Cell,
}

impl EditTab {

	pub fn rules_ui(&mut self, ctx: &Context) {
		SidePanel::new(Side::Left, "Rule Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				for (i, rule) in self.l_rules.iter().enumerate() {
					let (rect, response) = ui.allocate_at_least(Vec2::new(150.0, 40.0), Sense::click());

		            let visuals = ui.style().interact_selectable(&response, self.current_rule==i);

					ui.painter().rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);

					if response.clicked() {
						self.current_rule = i;
					}

					let painter = ui.painter_at(rect);
					
					painter.text(rect.left_center() + Vec2::new(5.0, 0.0), Align2::LEFT_CENTER, format!("rule {i}"), FontId::monospace(12.0), visuals.text_color());
				
					let mut thumb_rect = rect.shrink(5.0);
					thumb_rect.set_left(thumb_rect.right() - thumb_rect.height());
					
					let painter = ui.painter_at(thumb_rect);
					let scale = thumb_rect.width() / rule.width().max(rule.height()) as f32;
					for ([x, y], cell) in rule {
						if cell.same_type(&Cell::Empty) { continue; }
						let [x, y] = rule.pos_to_raw_pos([x, y]);
						painter.rect(
							Rect::from_min_size(
								pos2(x as f32 * scale, (rule.height() - 1 - y) as f32 * scale) + thumb_rect.min.to_vec2(), 
								vec2(scale, scale)), 
								0.0, 
								arr_to_col(cell_col(&cell)
							), 
							Stroke::NONE
						);
					}
				}
			});
	}

	fn tools_ui(&mut self, ctx: &Context) {
		SidePanel::new(Side::Right, "Tool Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
				
				if ui.button("Optimize").clicked() {
					self.l_rules[self.current_rule].contract_empty();
				}
				ui.separator();

				ui.horizontal(|ui| {
					ui.selectable_value(&mut self.tool, EditTool::Draw, "Draw");
					ui.selectable_value(&mut self.tool, EditTool::Erase, "Erase");
				});

				ui.separator();
				
				if self.tool == EditTool::Draw {
					if ui.radio(self.draw_cell.same_type(&Cell::Passive), "Passive").clicked() {
						self.draw_cell = Cell::Passive;
					}
					if ui.radio(self.draw_cell.same_type(&Cell::Stem(0, Direction::UP)), "Stem").clicked() {
						self.draw_cell = Cell::Stem(0, Direction::UP);
					}
					ui.separator();

					if let Cell::Stem(n, dir) = &mut self.draw_cell {
						ui.horizontal(|ui| {
							ui.add(DragValue::new(n).speed(0.1).clamp_range(0..=255));
							ui.label("Stem cell type");
						});
						ui.radio_value(dir, Direction::UP, "Up");
						ui.radio_value(dir, Direction::RIGHT, "Right");
						ui.radio_value(dir, Direction::DOWN, "Down");
						ui.radio_value(dir, Direction::LEFT, "Left");
						
						ui.separator();
					}
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
			draw_cell: Cell::Passive,
			current_rule: 0,
		}
    }

    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update();

		if is_mouse_button_down(MouseButton::Left) && can_use_mouse {
			let pos: [i32; 2] = self.controls.mouse_world.floor().as_ivec2().into();
			let pos = [pos[0] as isize, -pos[1] as isize];
			
			let rule = &mut self.l_rules[self.current_rule];
			
			if rule.contains(pos) || !self.draw_cell.same_type(&Cell::Empty) {
				rule.insert_cell(match self.tool {
					EditTool::Draw => self.draw_cell,
					EditTool::Erase => Cell::Empty,
				}, pos);
			}
		}

		
        set_camera(self.controls.camera());
		
		let pixel = (self.controls.camera().screen_to_world((0.0, 1.0).into()) - self.controls.camera().screen_to_world((0.0, 0.0).into())).y;
		draw_grid_lines(&self.l_rules[self.current_rule], pixel);
		draw_grid(&self.l_rules[self.current_rule]);    
	}

	fn draw_ui(&mut self, ctx: &Context) {
		self.rules_ui(ctx);
		self.tools_ui(ctx);
	}
}