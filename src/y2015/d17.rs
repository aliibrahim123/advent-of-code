use std::collections::HashMap;

fn find(
	containers: &Vec<i32>, ind: usize, remaining: i32, used: u32,
	combinations: &mut HashMap<u32, i32>,
) {
	let remaining = remaining - containers[ind];
	if remaining == 0 {
		combinations.entry(used).and_modify(|c| *c += 1).or_insert(1);
		combinations.entry(0).and_modify(|c| *c += 1);
		return;
	}
	if remaining < 0 {
		return;
	}

	for i in ind + 1..containers.len() {
		find(containers, i, remaining, used + 1, combinations);
	}
}

pub fn run(input: String) -> Option<()> {
	let containers = input.lines().filter(|l| !l.is_empty());
	let containers = containers.map(|l| l.parse().unwrap()).collect::<Vec<i32>>();
	let mut combinations = HashMap::new();
	combinations.insert(0, 0);

	for i in 0..containers.len() {
		find(&containers, i, 150, 1, &mut combinations);
	}

	println!("part1: {}", combinations[&0]);
	combinations.remove(&0);
	println!("part2: {}", combinations.iter().min_by_key(|(k, _)| *k)?.1);
	Some(())
}
