use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: String) -> Option<()> {
	let (replacments, molecule) = input.split_once("\n\n")?;
	let replacments =
		replacments.lines().map(|line| line.split_once(" => ").unwrap()).collect_vec();

	let mut seen = HashSet::new();
	for (from, to) in &replacments {
		for (ind, _) in molecule.match_indices(from) {
			let mut new_mol = molecule.to_string();
			new_mol.replace_range(ind..ind + from.len(), to);
			seen.insert(new_mol);
		}
	}

	println!("part1: {}", seen.len());

	let count_tokens = molecule.chars().filter(|c| c.is_uppercase()).count();
	let count_rn = molecule.matches("Rn").count();
	let count_ar = molecule.matches("Ar").count();
	let count_y = molecule.matches("Y").count();

	let steps = count_tokens - count_rn - count_ar - (2 * count_y) - 1;
	println!("part2: {steps}");
	Some(())
}
