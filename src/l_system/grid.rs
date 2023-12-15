use std::fmt::Display;

use rand::rngs::ThreadRng;

use crate::l_system::cell::{Cell, Direction};

#[derive(Clone)]
pub struct Grid {
	contents: Vec<Cell>,
	width: u32,
	height: u32,
	shift: [u32; 2], // -1 times the coordinates of the first cell. Alternatively (raw_x,raw_y) of (0,0)
}

impl Grid {

	pub fn from_string(string: &str, shift: [u32; 2]) -> Option<Grid> {
		let mut width = 0;
		let mut height = 0;
		let mut contents = Vec::new();

		for line in string.lines().rev() {
			let mut line_length = 0;
			for ch in line.chars() {
				contents.push(match ch {
					'.' => Cell::Empty,
					_ => Cell::Passive,
				});
				line_length += 1;
			}

			if width == 0 { width = line_length }
			else if width != line_length {
				return None;
			}
			height += 1;
		}

		Some(Grid::new(width, height, contents, shift))
	}

	pub fn new(width: u32, height: u32, contents: Vec<Cell>, shift: [u32; 2]) -> Self {
		assert!(width > shift[0] && height > shift[1]);
		assert!(width*height == contents.len() as u32);
		Self {
			width,
			height,
			contents: contents,
			shift,
		}
	}

	pub fn single(item: Cell) -> Self {
		Self {
			width: 1,
			height: 1,
			contents: vec![item],
			shift: [0, 0],
		}
	}

	pub fn horizontal(contents: &[Cell], shift: u32) -> Self {
		assert!(contents.len() as u32 > shift);
		Self {
			width: contents.len() as u32,
			height: 1,
			contents: Vec::from(contents),
			shift: [shift, 0],
		}
	}

	pub fn vertical(contents: Vec<Cell>, shift: u32) -> Self {
		assert!(contents.len() as u32 > shift);
		Self {
			width: 1,
			height: contents.len() as u32,
			contents: contents,
			shift: [0, shift],
		}
	}


	fn pos_to_index(&self, pos: [i32; 2]) -> usize {
		self.raw_pos_to_index(self.pos_to_raw_pos(pos))
	}

	fn raw_pos_to_index(&self, raw_pos: [u32; 2]) -> usize {
		(raw_pos[0] + raw_pos[1] * self.width) as usize
	}

	pub fn pos_to_raw_pos(&self, pos: [i32; 2]) -> [u32; 2] {
		[(pos[0] + self.shift[0] as i32) as u32, (pos[1] + self.shift[1] as i32) as u32]
	}

	#[allow(dead_code)]
	fn raw_pos_to_pos(&self, raw_pos: [u32; 2]) -> [i32; 2] {
		[raw_pos[0] as i32 - self.shift[0] as i32, raw_pos[1] as i32 - self.shift[1] as i32]
	}

	pub fn at_unchecked(&self, pos: [i32; 2]) -> Cell {
		self.contents[self.pos_to_index(pos)]
	}

	pub fn at_raw_unchecked(&self, raw_pos: [u32; 2]) -> Cell {
		self.contents[self.raw_pos_to_index(raw_pos)]
	}

	pub fn at(&self, pos: [i32; 2]) -> Cell {
		if !self.contains(pos) { return Cell::Empty; }
		self.contents[self.pos_to_index(pos)]
	}

	pub fn at_raw(&self, raw_pos: [u32; 2]) -> Cell {
		if !self.contains_raw(raw_pos) { return Cell::Empty; }
		self.contents[self.raw_pos_to_index(raw_pos)]
	}

	pub fn pos_to_other_pos(&self, pos: [i32; 2], other_pos: [i32; 2]) -> [i32; 2] {
		[pos[0] - other_pos[0], pos[1] - other_pos[1]]
	}

	pub fn contains(&self, pos: [i32; 2]) -> bool {
		(pos[0] + self.shift[0] as i32) >= 0 &&
		(pos[1] + self.shift[1] as i32) >= 0 &&
		(pos[0] + self.shift[0] as i32) < self.width as i32 &&
		(pos[1] + self.shift[1] as i32) < self.height as i32
	}

	pub fn contains_raw(&self, pos: [u32; 2]) -> bool {
		pos[0] < self.width &&
		pos[1] < self.height
	}

	pub fn insert_cell(&mut self, cell: Cell, pos: [i32; 2]) {
		if cell.same_type(&Cell::Empty) && !self.contains(pos) { return; }

		let top =    self.height - self.shift[1] - 1;
		let right =  self.width  - self.shift[0] - 1;
		let bottom = self.shift[1];
		let left =   self.shift[0];

		let expand_top =    i32::max(0,  pos[1] - top as i32   ) as u32;
		let expand_right =  i32::max(0,  pos[0] - right as i32 ) as u32;
		let expand_bottom = i32::max(0, -pos[1] - bottom as i32) as u32;
		let expand_left =   i32::max(0, -pos[0] - left as i32  ) as u32;

		if expand_top + expand_right + expand_bottom + expand_left > 0 {
			let new_width = expand_left + left + 1 + right + expand_right;
			let new_height = expand_top + top + 1 + bottom + expand_bottom;
			let new_shift = [self.shift[0] + expand_left, self.shift[1] + expand_bottom];
			
			let mut new_contents = Vec::with_capacity((new_width * new_height) as usize);

			for y in 0..new_height {
				for x in 0..new_width {
					if x < expand_left   || x >= new_width - expand_right ||
					   y < expand_bottom || y >= new_height - expand_top {
						new_contents.push(Cell::Empty);
					}
					else {
						new_contents.push(self.at_raw([x - expand_left, y - expand_bottom]));
					}
				}
			}

			self.contents = new_contents;
			self.width = new_width;
			self.height = new_height;
			self.shift = new_shift;
		}

		let i = self.pos_to_index(pos);
		self.contents[i] = cell;
	}

	pub fn insert(&mut self, other: &Grid, pos: [i32; 2], other_dir: Direction) {

		let top =    self.height - self.shift[1] - 1;
		let right =  self.width  - self.shift[0] - 1;
		let bottom = self.shift[1];
		let left =   self.shift[0];

		let mut other_top =    other.height - other.shift[1] - 1;
		let mut other_right =  other.width  - other.shift[0] - 1;
		let mut other_bottom = other.shift[1];
		let mut other_left =   other.shift[0];

		[other_top, other_left, other_bottom, other_right] = 
			other_dir.rotate_vals(other_top, other_left, other_bottom, other_right);

		let expand_top =   i32::max(0,  pos[1] + other_top as i32    - top as i32   ) as u32;
		let expand_right = i32::max(0,  pos[0] + other_right as i32  - right as i32 ) as u32;
		let expand_bottom =i32::max(0, -pos[1] + other_bottom as i32 - bottom as i32) as u32;
		let expand_left =  i32::max(0, -pos[0] + other_left as i32   - left as i32  ) as u32;

		if expand_top + expand_right + expand_bottom + expand_left > 0 {
			let new_width = expand_left + left + 1 + right + expand_right;
			let new_height = expand_top + top + 1 + bottom + expand_bottom;
			let new_shift = [self.shift[0] + expand_left, self.shift[1] + expand_bottom];
			
			let mut new_contents = Vec::with_capacity((new_width * new_height) as usize);

			for y in 0..new_height {
				for x in 0..new_width {
					if x < expand_left   || x >= new_width - expand_right ||
					   y < expand_bottom || y >= new_height - expand_top {
						new_contents.push(Cell::Empty);
					}
					else {
						new_contents.push(self.at_raw([x - expand_left, y - expand_bottom]));
					}
				}
			}

			self.contents = new_contents;
			self.width = new_width;
			self.height = new_height;
			self.shift = new_shift;
		}

		for ([x, y], cell) in other {
			match cell {
				Cell::Empty => {},
				cell => {
					let [x, y] = other_dir.rotate_coords([x, y]);
					let i = self.pos_to_index([x + pos[0], y + pos[1]]);
					self.contents[i] = other_dir.rotate_cell(cell);
				},
			}
		}
	}

	pub fn contract(&mut self, direction: Direction) -> bool {
		match direction {
			Direction::UP => {
				if self.height < 2 || self.shift[1] == self.height - 1 { return false; }

				let len = self.contents.len();
				let start = len - self.width as usize;
				self.contents.drain(start..len);
				self.height -= 1;
			},
			Direction::LEFT => {
				if self.width < 2 || self.shift[0] == 0 { return false; }

				let mut i = 0;
				self.contents.retain(|_| {
					let ret = i % self.width == 0;
					i += 1;
					!ret
				});

				self.shift[0] -= 1;
				self.width -= 1;
			},
			Direction::DOWN => {
				if self.height < 2 || self.shift[1] == 0 { return false; }

				self.contents.drain(0..self.width as usize);

				self.shift[1] -= 1;
				self.height -= 1;
			},
			Direction::RIGHT => {
				if self.width < 2 || self.shift[0] == self.width - 1 { return false; }

				let mut i = 0;
				self.contents.retain(|_| {
					let ret = i % self.width == self.width - 1;
					i += 1;
					!ret
				});
				self.width -= 1;
			},
		}

		assert!(self.shift[0] < self.width);
		assert!(self.shift[1] < self.height);
		assert!(self.width > 0);
		assert!(self.height > 0);
		assert!(self.height * self.width == self.contents.len() as u32);

		true
	}

	pub fn expand(&mut self, direction: Direction, rng: &mut ThreadRng, stem_types: u8) {
		match direction{
			Direction::UP => {
				for _ in 0..self.width {
					self.contents.push(Cell::random(rng, stem_types));
				}

				self.height += 1;
			},
			Direction::LEFT => {
				for i in (0..self.height).rev() {
					self.contents.insert((self.width * i) as usize, Cell::random(rng, stem_types));
				}

				self.shift[0] += 1;
				self.width += 1;
			},
			Direction::DOWN => {
				let mut new_contents = Vec::with_capacity(((self.height + 1) * self.width) as usize);
				for _ in 0..self.width {
					new_contents.push(Cell::random(rng, stem_types));
				}
				new_contents.append(&mut self.contents);
				self.contents = new_contents;

				self.shift[1] += 1;
				self.height += 1;
			},
			Direction::RIGHT => {
				for i in (0..self.height).rev() {
					self.contents.insert((self.width * (i+1)) as usize, Cell::random(rng, stem_types));
				}

				self.width += 1;
			},
		}
	}

	pub fn contract_empty(&mut self) {
		while self.try_contract_empty() {}
	}

	pub fn try_contract_empty(&mut self) -> bool {
		let mut top = true;
		let mut bottom = true;
		let mut left = true;
		let mut right = true;

		for x in 0..self.width {
			if let Cell::Empty = self.at_raw([x, 0]) {}
			else { bottom = false; }
			
			if let Cell::Empty = self.at_raw([x, self.height-1]) {}
			else { top = false; }
		}

		for y in 0..self.height {
			if let Cell::Empty = self.at_raw([0, y]) {}
			else { left = false; }
			
			if let Cell::Empty = self.at_raw([self.width-1, y]) {}
			else { right = false; }
		}

		if top 	  { top    = self.contract(Direction::UP); }
		if bottom { bottom = self.contract(Direction::DOWN); }
		if left   { left   = self.contract(Direction::LEFT); }
		if right  { right  = self.contract(Direction::RIGHT); }

		top || bottom || left || right
	}

	pub fn rotate(&mut self, dir: Direction) {
		let mut new = Grid::single(Cell::Empty);
		new.insert(&self, [0, 0], dir);
		*self = new;
	}

	pub fn clear(&mut self) {
		for c in &mut self.contents {
			*c = Cell::Empty;
		}
	}


	pub fn contents(&self) -> &Vec<Cell> {
		&self.contents
	}

	pub fn contents_mut(&mut self) -> &mut Vec<Cell> {
		&mut self.contents
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn shift(&self) -> [u32; 2] {
		self.shift
	}

	pub fn score_simmilarity(&self, other: &Self) -> f32 {
		let mut score = 0.0;

		for ([x, y], cell) in self {
			let equal = cell.same_type(&other.at([x, y]));
			let self_empty = cell.same_type(&Cell::Empty);
			if equal && !self_empty {
				score += 1.0;
			}
			else if !equal {
				score -= 1.0;
			}
		}
		for ([x, y], cell) in other {
			if self.contains([x, y]) { continue; }

			// we are outside self, so it is self.at([x, y]) is empty
			if !cell.same_type(&Cell::Empty) {
				score -= 1.0;
			}
		}
		
		score
	}


	pub fn serialize(&self) -> Vec<u8> {
		let mut data = Vec::with_capacity((self.width * self.height) as usize);

		data.extend_from_slice(&self.width.to_be_bytes());
		data.extend_from_slice(&self.height.to_be_bytes());
		data.extend_from_slice(&self.shift[0].to_be_bytes());
		data.extend_from_slice(&self.shift[1].to_be_bytes());

		for cell in &self.contents {
			match cell {
				Cell::Empty => data.push(0),
				Cell::Passive => data.push(1),
				Cell::Stem(n, dir) => data.extend_from_slice(&[2, *n, dir.to_byte()]),
			}
		}
		
		data
	}

	pub fn deserialize(data: &[u8]) -> Result<Self, ()> {
		if data.len() < 16 {
			return Err(());
		}

		let width = u32::from_be_bytes(data[0..4].try_into().unwrap());
		let height = u32::from_be_bytes(data[4..8].try_into().unwrap());
		let shift = [
			u32::from_be_bytes(data[8..12].try_into().unwrap()),
			u32::from_be_bytes(data[12..16].try_into().unwrap()),
		];

		let mut contents = Vec::with_capacity((width * height) as usize);

		let mut cursor = 16;
		while cursor < data.len() {
			match data[cursor] {
				0 => contents.push(Cell::Empty),
				1 => contents.push(Cell::Passive),
				2 => {
					if data.len() <= cursor+2 {
						return Err(());
					}

					let n = data[cursor+1];
					let dir = Direction::from_byte(data[cursor+2]);

					contents.push(Cell::Stem(n, dir));
					cursor += 2;
				},
				_ => return Err(()),
			}
			cursor += 1;
		}

		if contents.len() != (width * height) as usize {
			return Err(());
		}
		if !(width > shift[0] && height > shift[1]) {
			return Err(());
		}

		Ok(
			Self {
				contents,
				width,
				height,
				shift,
			}
		)
	}
}




impl<'a> IntoIterator for &'a Grid {
    type Item = ([i32; 2], Cell);

    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
			grid: &self,
			pos: 0,
		}
    }
}

pub struct GridIterator<'a> {
	grid: &'a Grid,
	pos: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = ([i32; 2], Cell);

    fn next(&mut self) -> Option<Self::Item> {
		if self.pos >= (self.grid.width * self.grid.height) as usize { return None; }

        let ret = self.grid.contents[self.pos];
		let x = self.pos % self.grid.width as usize;
		let y = self.pos / self.grid.width as usize;

		self.pos += 1;

		Some(([x as i32 - self.grid.shift[0] as i32, y as i32 - self.grid.shift[1] as i32], ret))
    }
}


impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", '\u{250C}')?;
		for i in 0..self.width {
			write!(f, "{0}{0}", if i == self.shift[0] {'\u{253C}'} else {'\u{2500}'})?;
		}
		writeln!(f, "{}", '\u{2510}')?;
		

		for y in (0..self.height).rev() {
			write!(f, "{}", if y == self.shift[1] {'\u{256A}'} else {'\u{2502}'})?;
			for x in 0..self.width {
				let cell = self.at_raw([x, y]);

				match cell {
					Cell::Stem(n, d) => write!(f, "{n}{d}")?,
					Cell::Passive => write!(f, "{0}{0}", '\u{2588}')?,
					Cell::Empty => write!(f, "  ")?,
				};
			}
			writeln!(f, "{}", if y == self.shift[1] {'\u{256A}'} else {'\u{2502}'})?;
		}
		
		write!(f, "{}", '\u{2514}')?;
		for i in 0..self.width {
			write!(f, "{0}{0}", if i == self.shift[0] {'\u{253C}'} else {'\u{2500}'})?;
		}
		write!(f, "{}", '\u{2518}')?;

		Ok(())
    }
}