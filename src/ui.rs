use egui_macroquad::egui::{Ui, WidgetText, Layout, Align, TextStyle, Vec2, Sense, Response, Label, RichText, Color32, Button, vec2, Rect, pos2, Stroke};
use soft_evolution::l_system::{grid::Grid, cell::Cell};

use crate::drawing::{arr_to_col, cell_col, stem_cell_col};

pub fn centered_button(ui: &mut Ui, size: Vec2, text: impl Into<WidgetText>) -> Response{
	let (rect, response) = ui.allocate_exact_size(size, Sense::click());
	
	let visuals = ui.style().interact(&response);
	ui.painter().rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);

	let text = text.into().into_galley(ui, None, size.x, TextStyle::Button);
	let text_pos = Layout::top_down(Align::Center)
		.align_size_within_rect(text.size(), rect.shrink(5.0))
		.min;
	text.paint_with_visuals(ui.painter(), text_pos, &visuals);

	response
}

fn round_rect_to_pixel(ui: &Ui, rect: Rect) -> Rect {
	Rect {
		min: pos2(floor_to_pixel(ui, rect.min.x), floor_to_pixel(ui, rect.min.y)),
		max: pos2(ceil_to_pixel(ui, rect.max.x), ceil_to_pixel(ui, rect.max.y)),
	}
	//vec2(round_to_pixel(ui, vec.x), round_to_pixel(ui, vec.y))
}

fn floor_to_pixel(ui: &Ui, point: f32) -> f32 {
	let pixels_per_point = ui.ctx().pixels_per_point();
	(point * pixels_per_point).floor() / pixels_per_point
}

fn ceil_to_pixel(ui: &Ui, point: f32) -> f32 {
	let pixels_per_point = ui.ctx().pixels_per_point();
	(point * pixels_per_point).ceil() / pixels_per_point
}

pub fn draw_grid_ui(ui: &mut Ui, grid: &Grid, mut rect: Rect) {
	let aspect = grid.width() as f32 / grid.height() as f32;
	if aspect > rect.aspect_ratio() {
		rect = rect.shrink2(vec2(0.0, (rect.height() - rect.width() / aspect)*0.5));
	}
	else {
		rect = rect.shrink2(vec2((rect.width() - rect.height() * aspect)*0.5, 0.0));
	}

	let scale = rect.width() / grid.width() as f32;
	for ([x, y], cell) in grid {
		if cell.same_type(&Cell::Empty) { continue; }
		let [x, y] = grid.pos_to_raw_pos([x, y]);
		ui.painter().rect(
			round_rect_to_pixel(ui, Rect::from_min_size(
				pos2(
					x as f32 * scale,
					(grid.height() - 1 - y) as f32 * scale,
				) + rect.min.to_vec2(),
				vec2(scale, scale)
			)),
				0.0, 
				arr_to_col(cell_col(&cell)
			), 
			Stroke::NONE
		);
	}
}

#[derive(PartialEq)]
pub enum RuleButtonResponse {
	None,
	Select,
	Delete,
	MoveUp,
	MoveDown,
	Duplicate,
}

pub fn rule_button(ui: &mut Ui, rule: &Grid, index: usize, num_rules: usize, selectible: bool) -> RuleButtonResponse {
	let mut resp = RuleButtonResponse::None;
	
	let (rect, big_response) = ui.allocate_exact_size(Vec2::new(150.0, 40.0), Sense::click());
	if big_response.clicked() {
		resp = RuleButtonResponse::Select;
	}
	let mut line_rect = rect.clone();
	line_rect.set_width(5.0);	

	let visuals = ui.style().interact_selectable(&big_response, selectible);
	ui.painter().rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);
	ui.painter().rect(line_rect, visuals.rounding, arr_to_col(stem_cell_col(index as u8)), Stroke::NONE);

	let mut ui = ui.child_ui(rect, Layout::left_to_right(Align::Center));
	ui.add_space(10.0);
	ui.add(Label::new(RichText::new(format!("{index}")).color(Color32::WHITE).strong()));

	let tmp = ui.style().spacing.item_spacing;
	ui.style_mut().spacing.item_spacing = vec2(2.0, 2.0);

	ui.vertical(|ui| {
		ui.add_space(1.0);
		if ui.add_enabled(index > 0, Button::new("\u{2B06}")).clicked() {
			resp = RuleButtonResponse::MoveUp;
		}
		if ui.add_enabled(index < num_rules-1, Button::new("\u{2B07}")).clicked() {
			resp = RuleButtonResponse::MoveDown;
		}
	});
	ui.vertical(|ui| {
		ui.add_space(1.0);
		if ui.add(Button::new("\u{1F5D0}")).clicked() {
			resp = RuleButtonResponse::Duplicate;
		}
		if ui.add_enabled(num_rules > 1, Button::new("\u{1F5D1}").fill(Color32::from_rgb(150, 0, 0))).clicked() {
			resp = RuleButtonResponse::Delete;
		}
	});

	ui.style_mut().spacing.item_spacing = tmp;

	let rect = ui.available_rect_before_wrap().shrink(5.0);
	draw_grid_ui(&mut ui, rule, rect);

	resp
}