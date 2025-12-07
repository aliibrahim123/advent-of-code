use std::cmp::{max, min};

use itertools::Itertools;

struct Dear {
	speed: u64,
	speed_dur: u64,
	rest_dur: u64,
}

#[derive(Default, Clone)]
struct DearState {
	dst: u64,
	cur_time: u64,
	is_resting: bool,
	points: u64,
}

pub fn run(input: String) -> Option<()> {
	let race_time = 2503;
	let mut dears = Vec::new();
	let mut best_dist = 0;

	for line in input.lines() {
		let (sent1, sent2) = line.split_once(", ").unwrap();
		let (_, _, _, speed, _, _, speed_dur, _) = sent1.split(' ').collect_tuple()?;
		let (_, _, _, _, _, rest_dur, _) = sent2.split(' ').collect_tuple()?;

		dears.push(Dear {
			speed: speed.parse::<u64>().ok()?,
			speed_dur: speed_dur.parse::<u64>().ok()?,
			rest_dur: rest_dur.parse::<u64>().ok()?,
		});
	}

	for Dear { speed, speed_dur, rest_dur, .. } in &dears {
		let (mut dst, mut time) = (0, 0);

		while time < race_time {
			dst += speed * min(*speed_dur, race_time - time);
			if race_time - time < *speed_dur {
				break;
			}
			time += speed_dur;
			time += rest_dur;
		}
		best_dist = max(best_dist, dst);
	}
	println!("part1: {best_dist}");

	let mut states: Vec<DearState> = vec![Default::default(); dears.len()];
	let mut best = 0;

	for _ in 0..race_time {
		for (i, dear) in dears.iter().enumerate() {
			let Dear { speed, speed_dur, rest_dur } = dear;
			let DearState { dst, cur_time, is_resting, .. } = &mut states[i];

			if !*is_resting {
				*dst += speed;
			}

			*cur_time += 1;
			if cur_time == if *is_resting { rest_dur } else { speed_dur } {
				*cur_time = 0;
				*is_resting = !*is_resting;
			}
		}

		let best_dst = states.iter().map(|s| s.dst).max()?;
		for state in &mut states {
			state.points += u64::from(state.dst == best_dst);
			best = max(best, state.points);
		}
	}

	println!("part2: {best}");
	Some(())
}
