use itertools::Itertools;

pub fn run(input: String) -> Option<()> {
	#[rustfmt::skip]
	let (row, col) = input
		.split(|c: char| !c.is_numeric()).filter_map(|v| v.parse::<u64>().ok()).collect_tuple()?;

	let side = row + col - 2;
	let index = (side * (side + 1)) / 2 + col;

	let (start_code, multiplier, modulus) = (20151125, 252533, 33554393);

	let factor = mod_pow(multiplier, index - 1, modulus);
	let code = (start_code * factor) % modulus;

	println!("part 1: {code}");

	Some(())
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
	if modulus == 1 {
		return 0;
	}
	let mut result = 1;
	base %= modulus;
	while exp > 0 {
		if exp % 2 == 1 {
			result = (result * base) % modulus;
		}
		base = (base * base) % modulus;
		exp /= 2;
	}
	result
}
