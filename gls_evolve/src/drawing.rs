use std::f32::consts::PI;

use egui_macroquad::{macroquad::prelude::*, egui::{Color32, emath::lerp}};
use soft_evolution::l_system::{grid::Grid, cell::{Cell, Direction}};


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

pub fn stem_cell_border_col(stem: u8) -> [f32; 4] {
	col_from_hsv(0.678 / PI * stem as f32, 0.6, 0.5)
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

pub fn draw_cell_rect(cell: Cell, x: f32, y: f32, radius: f32) {
	let (col, border) = match cell {
		Cell::Stem(n, _) => (Color::from(stem_cell_col(n)), Color::from(stem_cell_border_col(n))),
		Cell::Passive => (Color::new(0.5, 0.5, 0.5, 1.0), Color::new(0.25, 0.25, 0.25, 1.0)),
		Cell::Empty => (BLACK, BLACK),
	};

	let margin = (1.0 - radius) * 0.5;

	let inner_radius = radius * 0.8;
	let inner_margin = (1.0-inner_radius) * 0.5;

	draw_rectangle(x + margin, -y + margin, radius, radius, border);
	draw_rectangle(x + inner_margin, -y + inner_margin, inner_radius, inner_radius, col);

}

pub fn draw_cell(cell: Cell, x: f32, y: f32, radius: f32) {

	match cell {
		Cell::Stem(n, dir) => {
			draw_cell_rect(cell, x, y, radius);
			let text = format!("{n}");
			let dims = measure_text(&text, None, 16, 1.0);
			
			let scale = f32::min(0.5*radius/dims.width, 0.5*radius/dims.height);
			let text_x = x + (1.0 - dims.width*scale)*0.5;
			let text_y = -y + dims.height*0.5*scale + 0.5;
			
			draw_text_ex(&text, text_x, text_y, TextParams {
				font_scale: scale,
				..STEM_TEXT_PARAMS
			});

			let (mut v1, mut v2, mut v3) = ([-0.2, -0.3], [0.0, -0.45], [0.2, -0.3]);
			v1 = dir.unrotate_coords(v1);
			v2 = dir.unrotate_coords(v2);
			v3 = dir.unrotate_coords(v3);
			let v1 = vec2(0.5, 0.5) + Vec2::from(v1)*radius + vec2(x, -y);
			let v2 = vec2(0.5, 0.5) + Vec2::from(v2)*radius + vec2(x, -y);
			let v3 = vec2(0.5, 0.5) + Vec2::from(v3)*radius + vec2(x, -y);

			draw_triangle(v1, v2, v3, STEM_TEXT_PARAMS.color);
		},
		Cell::Passive => {
			draw_cell_rect(cell, x, y, radius);
		},
		Cell::Empty => {},
	};
}

pub fn draw_grid(grid: &Grid) {
	for ([x, y], cell) in grid {
		let x = x as f32;
		let y = y as f32;

		draw_cell(cell, x, y, 1.0);
	}
}

pub fn draw_grid_animated(grid: &Grid, prev_grid: &Grid, rule: &Grid, from: [i32; 2], from_dir: Direction, t: f32) {
	let t_fast = (t*1.5).min(1.0);

	let from_x = from[0] as f32;
	let from_y = from[1] as f32;
	
	for ([x, y], cell) in grid {
		let old_cell = prev_grid.at([x, y]);
		let changed = old_cell.same_type(&Cell::Empty) && !cell.same_type(&Cell::Empty);
		let placed = !rule.at(from_dir.unrotate_coords(grid.pos_to_other_pos([x, y], from))).same_type(&Cell::Empty);
		
		if !changed && placed {
			draw_cell(old_cell, x as f32, y as f32, 1.0-t_fast);
		}
		if !changed && !placed {
			draw_cell(cell, x as f32, y as f32, 1.0);
		}
		
	}
	for ([x, y], cell) in grid {
		let placed = !rule.at(from_dir.unrotate_coords(grid.pos_to_other_pos([x, y], from))).same_type(&Cell::Empty);

		if placed {
			let mut xf = x as f32;
			let mut yf = y as f32;

			xf = lerp(from_x..=xf, smoothstep(t_fast));
			yf = lerp(from_y..=yf, smoothstep(t_fast));
			draw_cell(cell, xf, yf, ease_out_back(t));
		}
	}
}

pub fn smoothstep(x: f32) -> f32 {
	3.0*x*x - 2.0*x*x*x
}

pub fn ease_out_back(x: f32) -> f32 {
	2.70158*x*x*x - 6.40316*x*x + 4.70158*x
}