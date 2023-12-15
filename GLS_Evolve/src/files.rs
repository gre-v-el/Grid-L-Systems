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

// pub fn save_rules(rules: &[Grid]) {
// 	let mut data = Vec::new();
// 	for rule in rules {
		
// 	}
// }