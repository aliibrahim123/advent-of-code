use std::cmp::{max, min};

#[derive(Clone, Copy)]
struct State {
	player_health: i16,
	mana: i32,
	mana_spend: u16,
	boss_health: i16,
	boss_dmg: i16,
	shild_countdown: u8,
	poison_countdown: u8,
	recharge_countdown: u8,
}

impl State {
	fn spend_mana(mut self, mana: i32) -> Self {
		self.mana -= mana;
		self.mana_spend += mana as u16;
		self
	}
}

fn simulate_player(mut state: State, is_hard: bool, best_win: &mut u16) {
	if is_hard {
		state.player_health -= 1;
		if state.player_health == 0 {
			return;
		}
	}

	simulate_effects(&mut state);
	if state.boss_health <= 0 {
		*best_win = min(*best_win, state.mana_spend);
		return;
	}

	let State { mana, shild_countdown, poison_countdown, recharge_countdown, .. } = state;
	if mana < 53 || *best_win <= state.mana_spend {
		return;
	}
	if mana >= 53 {
		simulate_boss(state.spend_mana(53), 4, is_hard, best_win);
	}
	if mana >= 73 {
		let state = State { player_health: state.player_health + 2, ..state }.spend_mana(73);
		simulate_boss(state, 2, is_hard, best_win);
	}
	if mana >= 113 && shild_countdown == 0 {
		simulate_boss(State { shild_countdown: 6, ..state }.spend_mana(113), 0, is_hard, best_win);
	}
	if mana >= 173 && poison_countdown == 0 {
		simulate_boss(State { poison_countdown: 6, ..state }.spend_mana(173), 0, is_hard, best_win);
	}
	if mana >= 229 && recharge_countdown == 0 {
		let state = State { recharge_countdown: 5, ..state }.spend_mana(229);
		simulate_boss(state, 0, is_hard, best_win);
	}
}

fn simulate_boss(mut state: State, dmg: i16, is_hard: bool, best_win: &mut u16) {
	let shild_active = state.shild_countdown != 0;
	simulate_effects(&mut state);
	state.boss_health -= dmg;

	if state.boss_health <= 0 {
		*best_win = min(*best_win, state.mana_spend);
		return;
	}

	state.player_health -= max(state.boss_dmg - if shild_active { 7 } else { 0 }, 1);

	if state.player_health <= 0 {
		return;
	}

	simulate_player(state, is_hard, best_win);
}

fn simulate_effects(state: &mut State) {
	let State { shild_countdown, poison_countdown, recharge_countdown, .. } = state;

	if *shild_countdown != 0 {
		*shild_countdown -= 1;
	}
	if *poison_countdown != 0 {
		*poison_countdown -= 1;
		state.boss_health -= 3;
	}
	if *recharge_countdown != 0 {
		*recharge_countdown -= 1;
		state.mana += 101;
	}
}

pub fn run(input: String) -> Option<()> {
	let mut lines = input.lines();
	let boss_health = lines.next()?.split_once(": ")?.1.parse().ok()?;
	let boss_dmg = lines.next()?.split_once(": ")?.1.parse().ok()?;

	#[rustfmt::skip]
	let state = State {
		player_health: 50, mana: 500, mana_spend: 0, boss_health, boss_dmg,
		shild_countdown: 0, poison_countdown: 0, recharge_countdown: 0,
	};

	let mut best_win = u16::MAX;
	simulate_player(state, false, &mut best_win);
	println!("part1: {best_win}");

	let mut best_win = u16::MAX;
	simulate_player(state, true, &mut best_win);
	println!("part2: {best_win}");
	Some(())
}
