use std::fmt::Display;

use rand::rngs::ThreadRng;

use crate::l_system::cell::{Cell, Direction};

#[derive(Clone)]
pub struct Grid {
	contents: Vec<Cell>,
	width: usize,
	height: usize,
	shift: [usize; 2], // -1 times the coordinates of the first cell. Alternatively (raw_x,raw_y) of (0,0)
}

impl Grid {

	pub fn from_string(string: &str, shift: [usize; 2]) -> Option<Grid> {
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

	pub fn new(width: usize, height: usize, contents: Vec<Cell>, shift: [usize; 2]) -> Self {
		assert!(width > shift[0] && height > shift[1]);
		assert!(width*height == contents.len());
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

	pub fn horizontal(contents: &[Cell], shift: usize) -> Self {
		assert!(contents.len() > shift);
		Self {
			width: contents.len(),
			height: 1,
			contents: Vec::from(contents),
			shift: [shift, 0],
		}
	}

	pub fn vertical(contents: &[Cell], shift: usize) -> Self {
		assert!(contents.len() > shift);
		Self {
			width: 1,
			height: contents.len(),
			contents: Vec::from(contents),
			shift: [0, shift],
		}
	}


	fn pos_to_index(&self, pos: [isize; 2]) -> usize {
		self.raw_pos_to_index(self.pos_to_raw_pos(pos))
	}

	fn raw_pos_to_index(&self, raw_pos: [usize; 2]) -> usize {
		raw_pos[0] + raw_pos[1] * self.width
	}

	fn pos_to_raw_pos(&self, pos: [isize; 2]) -> [usize; 2] {
		[(pos[0] + self.shift[0] as isize) as usize, (pos[1] + self.shift[1] as isize) as usize]
	}

	fn raw_pos_to_pos(&self, raw_pos: [usize; 2]) -> [isize; 2] {
		[raw_pos[0] as isize - self.shift[0] as isize, raw_pos[1] as isize - self.shift[1] as isize]
	}

	pub fn at_unchecked(&self, pos: [isize; 2]) -> Cell {
		self.contents[self.pos_to_index(pos)]
	}

	pub fn at_raw_unchecked(&self, raw_pos: [usize; 2]) -> Cell {
		self.contents[self.raw_pos_to_index(raw_pos)]
	}

	pub fn at(&self, pos: [isize; 2]) -> Cell {
		if !self.contains(pos) { return Cell::Empty; }
		self.contents[self.pos_to_index(pos)]
	}

	pub fn at_raw(&self, raw_pos: [usize; 2]) -> Cell {
		if !self.contains_raw(raw_pos) { return Cell::Empty; }
		self.contents[self.raw_pos_to_index(raw_pos)]
	}

	pub fn pos_to_other_pos(&self, pos: [isize; 2], other_pos: [isize; 2]) -> [isize; 2] {
		[pos[0] - other_pos[0], pos[1] - other_pos[1]]
	}

	pub fn contains(&self, pos: [isize; 2]) -> bool {
		(pos[0] + self.shift[0] as isize) >= 0 &&
		(pos[1] + self.shift[1] as isize) >= 0 &&
		(pos[0] + self.shift[0] as isize) < self.width as isize &&
		(pos[1] + self.shift[1] as isize) < self.height as isize
	}

	pub fn contains_raw(&self, pos: [usize; 2]) -> bool {
		pos[0] < self.width &&
		pos[1] < self.height
	}

	pub fn insert_cell(&mut self, cell: Cell, pos: [isize; 2]) {
		let top =    self.height - self.shift[1] - 1;
		let right =  self.width  - self.shift[0] - 1;
		let bottom = self.shift[1];
		let left =   self.shift[0];

		let expand_top =    isize::max(0,  pos[1] - top as isize   ) as usize;
		let expand_right =  isize::max(0,  pos[0] - right as isize ) as usize;
		let expand_bottom = isize::max(0, -pos[1] - bottom as isize) as usize;
		let expand_left =   isize::max(0, -pos[0] - left as isize  ) as usize;

		if expand_top + expand_right + expand_bottom + expand_left > 0 {
			let new_width = expand_left + left + 1 + right + expand_right;
			let new_height = expand_top + top + 1 + bottom + expand_bottom;
			let new_shift = [self.shift[0] + expand_left, self.shift[1] + expand_bottom];
			
			let mut new_contents = Vec::with_capacity(new_width * new_height);

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

		match cell {
			Cell::Empty => {},
			cell => {
				let i = self.pos_to_index(pos);
				self.contents[i] = cell;
			},
		}
	}

	pub fn insert(&mut self, other: &Grid, pos: [isize; 2], other_dir: Direction) {

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

		let expand_top =   isize::max(0,  pos[1] + other_top as isize    - top as isize   ) as usize;
		let expand_right = isize::max(0,  pos[0] + other_right as isize  - right as isize ) as usize;
		let expand_bottom =isize::max(0, -pos[1] + other_bottom as isize - bottom as isize) as usize;
		let expand_left =  isize::max(0, -pos[0] + other_left as isize   - left as isize  ) as usize;

		if expand_top + expand_right + expand_bottom + expand_left > 0 {
			let new_width = expand_left + left + 1 + right + expand_right;
			let new_height = expand_top + top + 1 + bottom + expand_bottom;
			let new_shift = [self.shift[0] + expand_left, self.shift[1] + expand_bottom];
			
			let mut new_contents = Vec::with_capacity(new_width * new_height);

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
					let [x, y] = other_dir.rotate_coords(x, y);
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
				let start = len - self.width;
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

				self.contents.drain(0..self.width);

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
		assert!(self.height * self.width == self.contents.len());

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
					self.contents.insert(self.width * i, Cell::random(rng, stem_types));
				}

				self.shift[0] += 1;
				self.width += 1;
			},
			Direction::DOWN => {
				let mut new_contents = Vec::with_capacity((self.height + 1) * self.width);
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
					self.contents.insert(self.width * (i+1), Cell::random(rng, stem_types));
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


	pub fn contents(&self) -> &Vec<Cell> {
		&self.contents
	}

	pub fn contents_mut(&mut self) -> &mut Vec<Cell> {
		&mut self.contents
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn shift(&self) -> [usize; 2] {
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
}




impl<'a> IntoIterator for &'a Grid {
    type Item = ([isize; 2], Cell);

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
    type Item = ([isize; 2], Cell);

    fn next(&mut self) -> Option<Self::Item> {
		if self.pos >= self.grid.width * self.grid.height { return None; }

        let ret = self.grid.contents[self.pos];
		let x = self.pos % self.grid.width;
		let y = self.pos / self.grid.width;

		self.pos += 1;

		Some(([x as isize - self.grid.shift[0] as isize, y as isize - self.grid.shift[1] as isize], ret))
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