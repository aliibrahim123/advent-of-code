use anyhow::Result;
use itertools::Itertools;

type Password = [u8; 8];

fn is_valid(input: &Password) -> bool {
	let cond1 = input.windows(3).any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2]);
	let cond2 = input.iter().all(|x| !matches!(x + b'a', b'i' | b'o' | b'l'));

	let mut pairs = 0;
	let mut window = input.windows(2);
	while let Some(x) = window.next() {
		if x[0] == x[1] {
			pairs += 1;
			window.next();
		}
	}

	cond1 && cond2 && pairs >= 2
}

fn add_one(input: &mut Password) {
	for i in (0..8).rev() {
		input[i] = (input[i] + 1) % 26;
		if input[i] != 0 {
			break;
		}
	}
}

fn next_valid(input: &mut Password) {
	while !is_valid(&input) {
		add_one(input);
	}
}

pub fn run(input: String) -> Result<()> {
	let input = input.trim();
	let mut input = input.bytes().map(|x| x - b'a').collect_array::<8>().unwrap();

	next_valid(&mut input);
	println!("part1: {}", input.iter().map(|x| x + b'a').map(char::from).collect::<String>());

	add_one(&mut input);
	next_valid(&mut input);
	println!("part2: {}", input.iter().map(|x| x + b'a').map(char::from).collect::<String>());
	Ok(())
}
