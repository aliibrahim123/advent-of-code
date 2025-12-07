use std::collections::HashMap;

pub fn run(input: String) -> Option<()> {
	let mut nice_strings = 0;
	'mainloop: for str in input.lines() {
		let mut vowels = 0;
		let mut last_char = '0';
		let mut double = false;
		for char in str.chars() {
			if matches!(char, 'a' | 'e' | 'i' | 'o' | 'u') {
				vowels += 1;
			}
			if char == last_char {
				double = true;
			}
			if matches!((last_char, char), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')) {
				continue 'mainloop;
			}
			last_char = char;
		}
		if vowels >= 3 && double {
			nice_strings += 1;
		}
	}
	println!("part1: {nice_strings}");

	let mut nice_strings = 0;
	for str in input.lines() {
		let chars = str.chars().collect::<Vec<_>>();
		let mut cond1 = false;
		let mut pairs = HashMap::new();
		for i in 0..chars.len() - 1 {
			let pair = (chars[i], chars[i + 1]);
			if let Some(ind) = pairs.get(&pair) {
				cond1 |= *ind + 1 != i
			} else {
				pairs.insert(pair, i);
			}
		}
		let mut cond2 = false;
		for i in 0..chars.len() - 2 {
			if chars[i] == chars[i + 2] {
				cond2 = true
			}
		}
		if cond1 && cond2 {
			nice_strings += 1;
		}
	}
	println!("part2: {nice_strings}");
	Some(())
}
