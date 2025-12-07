use std::mem;

use itertools::Itertools;

fn do_round(nb: &mut Vec<u8>, buf: &mut Vec<u8>) {
	buf.clear();
	buf.reserve(nb.len() * 2);

	for (count, &dg) in nb.iter().dedup_with_count() {
		buf.push((count as u8) + b'0');
		buf.push(dg);
	}
	mem::swap(nb, buf);
}

pub fn run(input: String) -> Option<()> {
	let mut nb = input.trim().bytes().collect::<Vec<_>>();
	let mut buf = Vec::new();

	for _ in 0..40 {
		do_round(&mut nb, &mut buf);
	}
	println!("part1: {}", nb.len());

	for _ in 0..10 {
		do_round(&mut nb, &mut buf);
	}
	println!("part2: {}", nb.len());
	Some(())
}
