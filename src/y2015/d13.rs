use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn calculate_best(relations: &HashMap<(&str, &str), i64>, names: &HashSet<&str>) -> Option<i64> {
	let mut best = 0;
	for perm in names.iter().permutations(names.len()) {
		let mut happiness = 0;
		for (&&who, &&whom) in perm.iter().tuple_windows() {
			happiness += relations.get(&(who, whom))?;
			happiness += relations.get(&(whom, who))?;
		}
		let (&&last, &&first) = (perm.first()?, perm.last()?);
		happiness += relations.get(&(first, last))?;
		happiness += relations.get(&(last, first))?;
		best = i64::max(best, happiness);
	}
	Some(best)
}

pub fn run(input: String) -> Option<()> {
	let mut relations = HashMap::<(&str, &str), i64>::new();
	let mut names = HashSet::new();

	for line in input.trim().lines() {
		let (who, _, action, amount, _, _, _, _, _, _, whom) =
			line[0..line.len() - 1].split(' ').collect_tuple()?;
		let amount = amount.parse::<i64>().ok()?;

		names.insert(who);
		relations.insert((&who, &whom), if action == "gain" { amount } else { -amount });
	}
	let best = calculate_best(&relations, &names)?;
	println!("Part 1: {best}");

	names.insert("Me");
	for name in names.iter() {
		relations.insert(("Me", name), 0);
		relations.insert((name, "Me"), 0);
	}

	let best = calculate_best(&relations, &names)?;
	println!("Part 2: {best}");
	Some(())
}
