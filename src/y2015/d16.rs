pub fn run(input: String) -> Option<()> {
	let mut sue1 = 0;
	let mut sue2 = 0;
	for line in input.trim().lines() {
		"Sue 315: cars: 2, cats: 5, pomeranians: 10";
		let (sue, props) = line.split_once(": ")?;
		let sue = sue.strip_prefix("Sue ")?;

		let mut props = props.split(", ").map(|prop| {
			let (key, val) = prop.split_once(": ").unwrap();
			(key, val.parse::<u64>().ok().unwrap())
		});

		if props.clone().all(|(key, value)| match key {
			"children" => 3,
			"cats" => 7,
			"samoyeds" => 2,
			"pomeranians" => 3,
			"akitas" => 0,
			"vizslas" => 0,
			"goldfish" => 5,
			"trees" => 3,
			"cars" => 2,
			"perfumes" => 1,
			_ => unreachable!(),
		} == value) {
			sue1 = sue.parse().ok()?;
		}

		if props.all(|(key, value)| match key {
			"children" => value == 3,
			"cats" => value > 7,
			"samoyeds" => value == 2,
			"pomeranians" => value < 3,
			"akitas" => value == 0,
			"vizslas" => value == 0,
			"goldfish" => value < 5,
			"trees" => value > 3,
			"cars" => value == 2,
			"perfumes" => value == 1,
			_ => unreachable!(),
		}) {
			sue2 = sue.parse().ok()?
		}
	}

	println!("part1: {sue1}");
	println!("part2: {sue2}");
	Some(())
}
