use std::fmt::Display;

use crate::cell::Cell;

pub struct Grid {
	contents: Vec<Cell>,
	width: usize,
	height: usize,
	shift: [usize; 2], // -1 times the coordinates of the first cell. Alternatively (raw_x,raw_y) of (0,0)
}

impl Grid {
	pub fn new(width: usize, height: usize, contents: &[Cell], shift: [usize; 2]) -> Self {
		assert!(width > shift[0] && height > shift[1]);
		Self {
			width,
			height,
			contents: Vec::from(contents),
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

	// fn raw_pos_to_pos(&self, raw_pos: [usize; 2]) -> [isize; 2] {
	// 	[raw_pos[0] as isize - self.shift[0] as isize, raw_pos[1] as isize - self.shift[1] as isize]
	// }

	pub fn at(&self, pos: [isize; 2]) -> Cell {
		self.contents[self.pos_to_index(pos)]
	}

	pub fn at_raw(&self, raw_pos: [usize; 2]) -> Cell {
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



	pub fn insert(&mut self, other: &Grid, pos: [isize; 2]) {

		let top =    self.height - self.shift[1] - 1;
		let right =  self.width  - self.shift[0] - 1;
		let bottom = self.shift[1];
		let left =   self.shift[0];

		let other_top =    other.height - other.shift[1] - 1;
		let other_right =  other.width  - other.shift[0] - 1;
		let other_bottom = other.shift[1];
		let other_left =   other.shift[0];

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
					let i = self.pos_to_index([x + pos[0], y + pos[1]]);
					self.contents[i] = cell;
				},
			}
		}
	}


	pub fn contents(&self) -> &Vec<Cell> {
		&self.contents
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
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
			write!(f, "{}", if i == self.shift[0] {'\u{253C}'} else {'\u{2500}'})?;
		}
		write!(f, "{}", '\u{2510}')?;
		

		write!(f, "\n{}", if 0 == self.shift[1] {'\u{253C}'} else {'\u{2502}'})?;

        for (i, cell) in self.contents.iter().enumerate() {
			match cell {
				Cell::Stem(n) => write!(f, "{n}")?,
				Cell::Passive => write!(f, "{}", '\u{2588}')?,
				Cell::Empty => write!(f, " ")?,
			};

			if (i + 1) % (self.width) == 0 {
				writeln!(f, "{}", if i / self.width == self.shift[1] {'\u{253C}'} else {'\u{2502}'})?;
				
				if i != self.contents.len() - 1 {
					write!(f, "{}", if (i+1) / self.width == self.shift[1] {'\u{253C}'} else {'\u{2502}'})?;
				}
			}
		}
		
		write!(f, "{}", '\u{2514}')?;
		for i in 0..self.width {
			write!(f, "{}", if i == self.shift[0] {'\u{253C}'} else {'\u{2500}'})?;
		}
		write!(f, "{}", '\u{2518}')?;

		Ok(())
    }
}