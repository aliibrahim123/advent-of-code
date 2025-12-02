use anyhow::Result;

pub fn run(input: String) -> Result<()> {
	let input = input.trim();
	let mut cur_nb = 0;
	let mut first_part = 0;
	let mut second_part = 0;
	loop {
		let hash = md5::compute(format!("{input}{cur_nb}"));
		if first_part == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0 {
			first_part = cur_nb;
		}
		if second_part == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
			second_part = cur_nb
		}
		if first_part != 0 && second_part != 0 {
			break;
		}
		cur_nb += 1;
	}
	println!("part 1: {first_part}");
	println!("part 2: {second_part}");
	Ok(())
}
