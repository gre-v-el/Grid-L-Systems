#[derive(Clone, Copy, Debug)]
pub enum Cell {
	Stem(u8),
	Passive,
	Empty,
}