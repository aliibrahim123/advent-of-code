use itertools::{Itertools, iproduct};
use std::cmp::{max, min};

use crate::utils::div_ciel;

pub fn run(input: String) -> Option<()> {
	let mut lines = input.lines();
	let boss_health = lines.next()?.split_once(": ")?.1.parse::<i32>().ok()?;
	let boss_dmg = lines.next()?.split_once(": ")?.1.parse::<i32>().ok()?;
	let boss_arm = lines.next()?.split_once(": ")?.1.parse::<i32>().ok()?;

	let weapons = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
	let armors = [(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];
	#[rustfmt::skip]
	let rings = [
		(0, 0, 0), (0, 0, 0), (25, 1, 0), (50, 2, 0),
		(100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3),
	];

	let mut best_win = i32::MAX;
	let mut worst_loss = i32::MIN;
	for (weapon, armor, rings) in iproduct!(weapons, armors, rings.iter().combinations(2)) {
		let [ring1, ring2] = rings.as_slice() else { unreachable!() };
		let player_dmg = weapon.1 + ring1.1 + ring2.1;
		let player_arm = armor.1 + ring1.2 + ring2.2;
		let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;

		let boss_life = div_ciel(boss_health, max(player_dmg - boss_arm, 1));
		let player_life = div_ciel(100, max(boss_dmg - player_arm, 1));

		if boss_life <= player_life {
			best_win = min(best_win, cost);
		} else {
			worst_loss = max(worst_loss, cost);
		}
	}
	println!("part1: {best_win}");
	println!("part2: {worst_loss}");
	Some(())
}
