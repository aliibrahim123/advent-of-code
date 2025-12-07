use itertools::Itertools;
use itertools::iproduct;
use std::cmp::max;
use std::cmp::min;

fn index_adj<T>(arr: &[T], i: usize) -> &[T] {
	&arr[max(i, 1) - 1..=min(i + 1, arr.len() - 1)]
}

fn simulate(grid_cur: &[[u8; 100]; 100], grid_next: &mut [[u8; 100]; 100]) {
	for (y, x) in iproduct!(0..100, 0..100) {
		let row = index_adj(grid_cur, y).iter();
		let sum: u8 = row.map(|col| index_adj(col, x).iter().sum::<u8>()).sum();

		grid_next[y][x] = match (grid_cur[y][x], sum) {
			(0, 3) | (1, 3) | (1, 4) => 1,
			_ => 0,
		};
	}
}

pub fn run(input: String) -> Option<()> {
	let lines = input.trim().lines();
	let lines =
		lines.map(|l| l.chars().map(|c| u8::from(c == '#')).collect_array::<100>().unwrap());

	let initial_grid = lines.collect_array::<100>()?;

	let mut grid_cur = initial_grid.clone();
	let mut grid_next = [[0u8; 100]; 100];

	for _ in 0..100 {
		simulate(&grid_cur, &mut grid_next);
		std::mem::swap(&mut grid_cur, &mut grid_next);
	}

	let sum = grid_cur.iter().flatten().map(|v| *v as u32).sum::<u32>();
	println!("part1: {sum}");

	grid_cur = initial_grid;
	grid_cur[0][0] = 1;
	grid_cur[0][99] = 1;
	grid_cur[99][0] = 1;
	grid_cur[99][99] = 1;
	for _ in 0..100 {
		simulate(&grid_cur, &mut grid_next);
		grid_next[0][0] = 1;
		grid_next[0][99] = 1;
		grid_next[99][0] = 1;
		grid_next[99][99] = 1;
		std::mem::swap(&mut grid_cur, &mut grid_next);
	}

	let sum = grid_cur.iter().flatten().map(|v| *v as u32).sum::<u32>();
	println!("part2: {sum}");

	Some(())
}
