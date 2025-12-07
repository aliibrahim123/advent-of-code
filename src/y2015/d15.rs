use std::cmp::max;

use itertools::Itertools;

struct Ingredient {
	capacity: i64,
	durability: i64,
	flavor: i64,
	texture: i64,
	calories: i64,
}

pub fn run(input: String) -> Option<()> {
	let mut ingredients = Vec::new();
	for line in input.trim().lines() {
		"Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5";
		let (_, _, capacity, _, durability, _, flavor, _, texture, _, calories) =
			line.split(' ').collect_tuple()?;

		ingredients.push(Ingredient {
			capacity: capacity[0..capacity.len() - 1].parse().ok()?,
			durability: durability[0..durability.len() - 1].parse().ok()?,
			flavor: flavor[0..flavor.len() - 1].parse().ok()?,
			texture: texture[0..texture.len() - 1].parse().ok()?,
			calories: calories.parse().ok()?,
		});
	}

	let mut best_ever = 0;
	let mut best_healthy = 0;
	for a in 0..100 {
		for b in 0..100 - a {
			for c in 0..100 - a - b {
				let d = 100 - a - b - c;

				let cap = a * ingredients[0].capacity
					+ b * ingredients[1].capacity
					+ c * ingredients[2].capacity
					+ d * ingredients[3].capacity;
				let dur = a * ingredients[0].durability
					+ b * ingredients[1].durability
					+ c * ingredients[2].durability
					+ d * ingredients[3].durability;
				let fla = a * ingredients[0].flavor
					+ b * ingredients[1].flavor
					+ c * ingredients[2].flavor
					+ d * ingredients[3].flavor;
				let tex = a * ingredients[0].texture
					+ b * ingredients[1].texture
					+ c * ingredients[2].texture
					+ d * ingredients[3].texture;
				let cal = a * ingredients[0].calories
					+ b * ingredients[1].calories
					+ c * ingredients[2].calories
					+ d * ingredients[3].calories;

				if cap <= 0 || dur <= 0 || fla <= 0 || tex <= 0 {
					continue;
				}

				let score = cap * dur * fla * tex;
				best_ever = max(best_ever, score);

				if cal == 500 {
					best_healthy = max(best_healthy, score);
				}
			}
		}
	}
	println!("Part 1: {best_ever}");
	println!("Part 1: {best_healthy}");
	Some(())
}
