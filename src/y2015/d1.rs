use anyhow::Result;

pub fn run(input: String) -> Result<()> {
	let mut floor = 0;
	for char in input.chars() {
		floor += if char == '(' { 1 } else { -1 };
	}
	println!("part 1: {floor}");

	let mut floor = 0;
	for (ind, char) in input.chars().enumerate() {
		floor += if char == '(' { 1 } else { -1 };
		if floor == -1 {
			println!("part 2: {}", ind + 1);
			break;
		}
	}
	Ok(())
}
