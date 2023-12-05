use egui_macroquad::{macroquad::prelude::*, egui::{Context, Window, DragValue, SidePanel, panel::Side}};
use soft_evolution::l_system::{grid::Grid, cell::{Cell, Direction}};

use crate::controls::Controls;

const STEM_TEXT_PARAMS: TextParams = TextParams {
	font: None,
	font_size: 16,
	font_scale: 1.0,
	font_scale_aspect: 1.0,
	rotation: 0.0,
	color: RED,
};

pub fn draw_grid_lines(grid: &Grid, width: f32) {
	let shift = grid.shift();
	let shift = [shift[0] as f32, shift[1] as f32];

	for x in 0..=grid.width() {
		// draw_line(x as f32 - shift[0], -shift[1], x as f32 - shift[0], shift[1] - grid.height() as f32, width, GRAY);
		let x = x as f32;
		draw_line(x - shift[0], shift[1] + 1.0, x - shift[0], shift[1] - grid.height() as f32 + 1.0, width, GRAY);
	}
	for y in 0..=grid.height() {

	}
}

pub fn draw_grid(grid: &Grid) {
	for ([x, y], cell) in grid {
		match cell {
			Cell::Stem(n, dir) => {
				draw_rectangle(x as f32, -y as f32, 1.0, 1.0, WHITE);
				let text = format!("{n}{dir}");
				let dims = measure_text(&text, None, 16, 1.0);
				
				let scale = f32::min(1.0/dims.width, 1.0/dims.height);
				let text_x = x as f32;
				let text_y = -y as f32 + dims.height*0.5*scale + 0.5;
				
				draw_text_ex(&text, text_x, text_y, TextParams {
					font_scale: scale,
					..STEM_TEXT_PARAMS
				});
			},
			Cell::Passive => {
				draw_rectangle(x as f32, -y as f32, 1.0, 1.0, GRAY);
			},
			Cell::Empty => {},
		};
	}
}

pub trait Tab {
	fn new() -> Self where Self: Sized;
	fn frame(&mut self, can_use_mouse: bool);
	fn draw_ui(&mut self, ctx: &Context);
}

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

impl Tab for EditTab {
    fn new() -> Self {
        Self {
			controls: Controls::new(),
			l_rules: vec![
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
				rule.insert_cell(self.draw_cell, pos);
			}
		}

		
        set_camera(self.controls.camera());
		
		let pixel = (self.controls.camera().screen_to_world((0.0, 1.0).into()) - self.controls.camera().screen_to_world((0.0, 0.0).into())).y;
		draw_grid_lines(&self.l_rules[0], pixel);
		draw_grid(&self.l_rules[0]);    
	}

    fn draw_ui(&mut self, ctx: &Context) {
		SidePanel::new(Side::Right, "Tool Choice")
			.resizable(false)
			.default_width(150.0)
			.show(ctx, |ui| {
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
					}
				}
			});
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
	
    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update();
		set_camera(self.controls.camera());
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

    fn frame(&mut self, can_use_mouse: bool) {
		self.controls.update();
		set_camera(self.controls.camera());
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