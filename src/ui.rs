use egui_macroquad::egui::{Ui, WidgetText, Layout, Align, TextStyle, Vec2, Sense, Response};

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