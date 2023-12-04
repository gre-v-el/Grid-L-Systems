use crate::tabs::{Tab, EditTab, EvolveTab, GrowTab};
use egui_macroquad::{egui, macroquad::prelude::*};

pub struct State {
	current_tab: usize,
	tabs: [Box<dyn Tab>; 3],
}

impl State {
	pub fn new() -> Self {
		Self {
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
		tab.update();

		egui_macroquad::ui(|ctx| {
			egui::TopBottomPanel::top("tabs").exact_height(20.0).show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.selectable_value(&mut self.current_tab, 0, "Edit");
					ui.selectable_value(&mut self.current_tab, 1, "Evolve");
					ui.selectable_value(&mut self.current_tab, 2, "Grow");
				});
			});

			tab.draw_ui(&ctx);
		});	
		egui_macroquad::draw();

		tab.set_camera();
		tab.draw_scene();
	}
}