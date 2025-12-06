use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

pub fn run(input: String) -> Result<()> {
	let mut distances = HashMap::new();
	let mut cities = HashSet::new();
	for line in input.trim().lines() {
		let line = line.trim();

		let (from, rest) = line.split_once(" to ").unwrap();
		let (to, dst) = rest.split_once(" = ").unwrap();
		let dst: u64 = dst.parse().unwrap();
		distances.insert((from, to), dst);
		distances.insert((to, from), dst);
		cities.insert(from);
		cities.insert(to);
	}

	let mut shortest_dst = u64::MAX;
	let mut longest_dst = u64::MIN;
	for perm in cities.iter().permutations(cities.len()) {
		let mut dst = 0;
		for i in 0..perm.len() - 1 {
			dst += distances[&(*perm[i], *perm[i + 1])];
		}
		shortest_dst = u64::min(shortest_dst, dst);
		longest_dst = u64::max(longest_dst, dst);
	}

	println!("part1: {}", shortest_dst);
	println!("part1: {}", longest_dst);
	Ok(())
}
