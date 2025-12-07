pub fn run(input: String) -> Option<()> {
	let mut paper = 0;
	let mut ribbon = 0;
	for line in input.lines() {
		let dim = line.split("x").map(|c| u64::from_str_radix(c, 10).unwrap()).collect::<Vec<_>>();
		let (l, w, h) = (dim[0], dim[1], dim[2]);
		let shortests = if l > w {
			if l > h { (h, w) } else { (w, l) }
		} else {
			if w > h { (h, l) } else { (l, w) }
		};
		let main_paper = 2 * l * w + 2 * w * h + 2 * h * l;
		let extra_paper = shortests.0 * shortests.1;
		paper += main_paper + extra_paper;

		let main_ribbon = 2 * (shortests.0 + shortests.1);
		let extra_ribbon = l * w * h;
		ribbon += main_ribbon + extra_ribbon;
	}
	println!("part 1: {paper}");
	println!("part 2: {ribbon}");
	Some(())
}
