use egui_macroquad::egui::{self, Context};

use crate::{edit_tab::EditTab, evolve_tab::EvolveTab, grow_tab::GrowTab};

pub trait Tab {
	fn new() -> Self where Self: Sized;
	fn frame(&mut self, can_use_mouse: bool);
	fn draw_ui(&mut self, ctx: &Context);
}



pub struct State {
	is_ui_using_mouse: bool,
	current_tab: usize,
	tabs: [Box<dyn Tab>; 3],
}

impl State {
	pub fn new() -> Self {
		Self {
			is_ui_using_mouse: false,
			current_tab: 0,
			tabs: [
				Box::new(EditTab::new()),
				Box::new(EvolveTab::new()),
				Box::new(GrowTab::new()),
			],
		}
	}

	pub fn frame(&mut self) {

		let tab = self.tabs[self.current_tab].as_mut();
		tab.frame(!self.is_ui_using_mouse);

		egui_macroquad::ui(|ctx| {
			egui::TopBottomPanel::top("tabs").exact_height(20.0).show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.selectable_value(&mut self.current_tab, 0, "Edit");
					ui.selectable_value(&mut self.current_tab, 1, "Evolve");
					ui.selectable_value(&mut self.current_tab, 2, "Grow");
				});
			});

			tab.draw_ui(&ctx);

			self.is_ui_using_mouse = ctx.is_pointer_over_area();
		});
		egui_macroquad::draw();

	}
}