use std::{fs::{self, read_dir}, path::PathBuf};

use soft_evolution::l_system::grid::Grid;

pub fn is_alphanumeric(s: &str) -> bool {
	let mut alphanumeric = s.is_ascii();
	if alphanumeric {
		for ch in s.chars() {
			if !ch.is_ascii_alphanumeric() {
				alphanumeric = false;
				break;
			}
		}
	}

	alphanumeric
}

pub fn save_rules(rules: &[Grid], filename: &str) -> Result<(), std::io::Error> {
	let mut data = Vec::new();
	for rule in rules {
		data.extend(rule.serialize())
	}
	fs::write(PathBuf::from(format!("./saves/{}.gls", filename)), data)
}

pub fn load_rules(filename: &str) -> Result<Vec<Grid>, String> {
	let mut rules = Vec::new();
	let data = fs::read(PathBuf::from(format!("./saves/{}.gls", filename)));
	if let Err(e) = data {
		return Err(e.to_string());
	}
	let data = data.unwrap();

	let mut cursor = 0;
	while cursor < data.len() {
		let res = Grid::deserialize(&data[cursor..]);
		if let Err(_) = res {
			return Err("Invalid file".into());
		}

		let (grid, c) = res.unwrap();
		rules.push(grid);
		cursor += c;
	}

	Ok(rules)
}

pub fn get_filenames() -> Vec<String> {
	let mut files = Vec::new();

	if let Ok(iter) = read_dir("./saves/") {
		for file in iter {
			if file.is_err() { continue; }
			let file = file.unwrap();

			let metadata = file.metadata();
			if metadata.is_err() {continue;}
			let metadata = metadata.unwrap();
			
			if !metadata.is_file() {  continue; }
			let path = file.path();
			let name = path.file_stem();
			let extension = path.extension();
			
			if name.is_none() { continue; }
			if extension.is_none() { continue; }

			let name = name.unwrap().to_owned().into_string();
			let extension = extension.unwrap().to_owned().into_string();
			if name.is_err() { continue; }
			if extension.is_err() { continue; }

			let name = name.unwrap();
			let extension = extension.unwrap();

			if !is_alphanumeric(&name) { continue; }
			if extension != "gls" { continue; }

			files.push(name);
		}
	}

	files
}