use itertools::Itertools;

pub fn run(input: String) -> Option<()> {
	let mut res = 0;
	for line in input.lines() {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		let src_len = line.len();

		let mut chars = line[1..src_len - 1].chars();
		let mut enc_len = 0;
		while let Some(char) = chars.next() {
			if char == '\\' {
				if chars.next()? == 'x' {
					_ = chars.next_tuple::<(_, _)>()
				}
			}
			enc_len += 1;
		}

		res += src_len - enc_len;
	}
	println!("part1: {}", res);

	let mut res = 0;
	for line in input.lines() {
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		let src_len = line.len();

		let mut enc_len = 2;
		for char in line.chars() {
			enc_len += if matches!(char, '\\' | '"') { 2 } else { 1 };
		}

		res += enc_len - src_len;
	}
	println!("part2: {}", res);
	Some(())
}
