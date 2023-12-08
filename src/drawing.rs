use std::f32::consts::PI;

use egui_macroquad::{macroquad::prelude::*, egui::Color32};
use soft_evolution::l_system::{grid::Grid, cell::Cell};


const GRID_COL: Color = color_u8!(58, 58, 58, 255);

const STEM_TEXT_PARAMS: TextParams = TextParams {
	font: None,
	font_size: 16,
	font_scale: 1.0,
	font_scale_aspect: 1.0,
	rotation: 0.0,
	color: BLACK,
};

pub fn col_from_hsv(hue: f32, saturation: f32, value: f32) -> [f32; 4] {

	let hue = hue + hue.floor();
	let hue = hue % 1.0;
	let saturation = saturation.clamp(0.0, 1.0);
	let value = value.clamp(0.0, 1.0);

    let h = (hue * 6.0) as i32;
    let f = hue * 6.0 - h as f32;
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - f * saturation);
    let t = value * (1.0 - (1.0 - f) * saturation);

    match h {
      0 => [value, t, p, 1.0],
      1 => [q, value, p, 1.0],
      2 => [p, value, t, 1.0],
      3 => [p, q, value, 1.0],
      4 => [t, p, value, 1.0],
      5 => [value, p, q, 1.0],
	  _ => unreachable!()
    }
}

pub fn stem_cell_col(stem: u8) -> [f32; 4] {
	col_from_hsv(0.678 / PI * stem as f32, 0.6, 1.0)
}

pub fn cell_col(cell: &Cell) -> [f32; 4] {
	match cell {
		Cell::Stem(n, _) => stem_cell_col(*n),
		Cell::Passive => [0.5, 0.5, 0.5, 1.0],
		Cell::Empty => [0.0, 0.0, 0.0, 1.0],
	}
}

pub fn arr_to_col(col: [f32; 4]) -> Color32 {
	Color32::from_rgba_premultiplied(
		(255.0 * col[0]) as u8, 
		(255.0 * col[1]) as u8, 
		(255.0 * col[2]) as u8, 
		(255.0 * col[3]) as u8
	)
}

pub fn pixel_width(camera: &Camera2D) -> f32 {
	(camera.screen_to_world((0.0, 1.0).into()) - camera.screen_to_world((0.0, 0.0).into())).y
}

pub fn draw_grid_lines(grid: &Grid, pixel: f32) {
	let shift = grid.shift();
	let shift = [shift[0] as f32, shift[1] as f32];

	for x in 0..=grid.width() {
		let x = x as f32;
		draw_line(x - shift[0], shift[1] + 1.0, x - shift[0], shift[1] - grid.height() as f32 + 1.0, pixel, GRID_COL);
	}
	for y in 0..=grid.height() {
		let y = y as f32;
		draw_line(-shift[0], -y + shift[1] + 1.0, grid.width() as f32 - shift[0], -y + shift[1] as f32 + 1.0, pixel, GRID_COL);
	}
}

pub fn draw_grid_axes(grid: &Grid, pixel: f32) {
	let shift = grid.shift();
	let shift = [shift[0] as f32, shift[1] as f32];

	draw_line(0.5, shift[1] + 1.0, 0.5, shift[1] - grid.height() as f32 + 1.0, pixel * 2.0, Color::new(0.0, 0.4, 0.0, 1.0));
	draw_line(-shift[0], 0.5, -shift[0] + grid.width() as f32, 0.5, pixel * 2.0, Color::new(0.4, 0.0, 0.0, 1.0));
}

pub fn draw_grid(grid: &Grid) {
	for ([x, y], cell) in grid {
		let x = x as f32;
		let y = y as f32;
		match cell {
			Cell::Stem(n, dir) => {
				draw_rectangle(x, -y, 1.0, 1.0, Color::from(stem_cell_col(n)));
				let text = format!("{n}");
				let dims = measure_text(&text, None, 16, 1.0);
				
				let scale = f32::min(0.5/dims.width, 0.5/dims.height);
				let text_x = x + (1.0 - dims.width*scale)*0.5;
				let text_y = -y + dims.height*0.5*scale + 0.5;
				
				draw_text_ex(&text, text_x, text_y, TextParams {
					font_scale: scale,
					..STEM_TEXT_PARAMS
				});

				let (mut v1, mut v2, mut v3) = ([-0.2, -0.3], [0.0, -0.45], [0.2, -0.3]);
				v1 = dir.rotate_coords(v1);
				v2 = dir.rotate_coords(v2);
				v3 = dir.rotate_coords(v3);
				v1[0] = -v1[0];
				v2[0] = -v2[0];
				v3[0] = -v3[0];
				let v1 = vec2(0.5, 0.5) + Vec2::from(v1) + vec2(x, -y);
				let v2 = vec2(0.5, 0.5) + Vec2::from(v2) + vec2(x, -y);
				let v3 = vec2(0.5, 0.5) + Vec2::from(v3) + vec2(x, -y);

				draw_triangle(v1, v2, v3, STEM_TEXT_PARAMS.color);
			},
			Cell::Passive => {
				draw_rectangle(x, -y, 1.0, 1.0, GRAY);
			},
			Cell::Empty => {},
		};
	}
}
