use std::cmp::max;

use anyhow::Result;

use crate::utils::OptionExt;

enum Command {
	Off,
	On,
	Toggle,
}

fn parse(inp: &str) -> Result<(Command, (usize, usize), (usize, usize))> {
	let mut parts = inp.split(' ');
	let command = match parts.next() {
		Some("turn") => match parts.next() {
			Some("on") => Command::On,
			Some("off") => Command::Off,
			_ => unreachable!(),
		},
		Some("toggle") => Command::Toggle,
		_ => unreachable!(),
	};

	let (x1, y1) = parts.next().ok()?.split_once(',').ok()?;
	let (x1, y1) = (usize::from_str_radix(x1, 10)?, usize::from_str_radix(y1, 10)?);
	parts.next();
	let (x2, y2) = parts.next().ok()?.split_once(',').ok()?;
	let (x2, y2) = (usize::from_str_radix(x2, 10)?, usize::from_str_radix(y2, 10)?);

	Ok((command, (x1, y1), (x2, y2)))
}

pub fn run(input: String) -> Result<()> {
	let mut lights = [[false; 1000]; 1000];

	for line in input.lines() {
		let (command, (x1, y1), (x2, y2)) = parse(line)?;

		for x in x1..=x2 {
			for y in y1..=y2 {
				match command {
					Command::On => lights[x][y] = true,
					Command::Off => lights[x][y] = false,
					Command::Toggle => lights[x][y] = !lights[x][y],
				}
			}
		}
	}
	let lit = lights.iter().flatten().filter(|&&lit| lit).count();
	println!("part 1: {lit}");

	let mut lights = [[0i32; 1000]; 1000];

	for line in input.lines() {
		let (command, (x1, y1), (x2, y2)) = parse(line)?;

		for x in x1..=x2 {
			for y in y1..=y2 {
				match command {
					Command::On => lights[x][y] += 1,
					Command::Off => lights[x][y] = max(0, lights[x][y] - 1),
					Command::Toggle => lights[x][y] += 2,
				}
			}
		}
	}
	let lit: i32 = lights.iter().flatten().sum();
	println!("part 2: {lit}");

	Ok(())
}
