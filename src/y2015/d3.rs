use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

pub fn run(input: String) -> Result<()> {
	let mut visited_houses = HashSet::new();
	let mut cur_house = (0, 0);
	visited_houses.insert(cur_house);
	for char in input.chars() {
		match char {
			'^' => cur_house.1 += 1,
			'v' => cur_house.1 -= 1,
			'>' => cur_house.0 += 1,
			'<' => cur_house.0 -= 1,
			_ => unreachable!(),
		};
		visited_houses.insert(cur_house);
	}
	println!("part 1: {}", visited_houses.len());

	let mut visited_houses = HashSet::new();
	let mut cur_house = (0, 0);
	let mut cur_house_robo = (0, 0);
	visited_houses.insert(cur_house);
	for (char1, char2) in input.chars().chunks(2).into_iter().map(|c| c.collect_tuple().unwrap()) {
		match char1 {
			'^' => cur_house.1 += 1,
			'v' => cur_house.1 -= 1,
			'>' => cur_house.0 += 1,
			'<' => cur_house.0 -= 1,
			_ => unreachable!(),
		};
		match char2 {
			'^' => cur_house_robo.1 += 1,
			'v' => cur_house_robo.1 -= 1,
			'>' => cur_house_robo.0 += 1,
			'<' => cur_house_robo.0 -= 1,
			_ => unreachable!(),
		}
		visited_houses.insert(cur_house);
		visited_houses.insert(cur_house_robo);
	}
	println!("part 2: {}", visited_houses.len());
	Ok(())
}
