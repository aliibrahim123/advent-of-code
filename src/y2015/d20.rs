pub fn run(input: String) -> Option<()> {
	let target = input.trim().parse().unwrap();

	let limit = target / 10;

	let mut houses = vec![0; limit];

	for elf in 1..limit {
		for house in (elf..limit).step_by(elf) {
			houses[house] += elf * 10;
		}
	}

	let house = houses.iter().position(|&p| p >= target)?;
	println!("part1: {house}");

	let mut houses = vec![0; limit];

	for elf in 1..limit {
		for house in (elf..limit).step_by(elf).take(50) {
			houses[house] += elf * 11;
		}
	}

	let house = houses.iter().position(|&p| p >= target)?;
	println!("part2: {house}");
	Some(())
}
