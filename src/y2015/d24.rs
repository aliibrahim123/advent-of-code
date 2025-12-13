use itertools::Itertools;

fn solve(weights: &[u64], target: u64) -> u64 {
	for len in 0..weights.len() {
		#[rustfmt::skip]
		let mut candidates = weights.iter().combinations(len)
			.filter(|combo| combo.iter().copied().sum::<u64>() == target)
			.map(|combo| combo.iter().copied().product::<u64>())
			.collect_vec();

		if !candidates.is_empty() {
			candidates.sort();
			return candidates[0];
		}
	}
	0
}

pub fn run(input: String) -> Option<()> {
	let weights = input.lines().filter_map(|v| v.parse::<u64>().ok()).collect_vec();

	let total_weight: u64 = weights.iter().sum();

	println!("part1: {}", solve(&weights, total_weight / 3));
	println!("part2: {}", solve(&weights, total_weight / 4));

	Some(())
}
