use std::collections::HashMap;

use anyhow::Result;

use crate::utils::StrExt;

enum Op {
	Move,
	Not,
	And,
	Or,
	LShift,
	RShift,
}
struct Gate<'a> {
	op: Op,
	a: &'a str,
	b: &'a str,
}

fn read<'a>(
	input: &'a str, gates: &HashMap<&'a str, Gate<'a>>, wires: &mut HashMap<&'a str, u16>,
) -> u16 {
	if input.first_char().is_digit(10) {
		return input.parse::<u16>().unwrap();
	}
	match wires.get(input) {
		Some(v) => *v,
		None => solve(input, gates, wires),
	}
}

fn solve<'a>(
	wire: &'a str, gates: &HashMap<&'a str, Gate<'a>>, wires: &mut HashMap<&'a str, u16>,
) -> u16 {
	let gate = &gates[wire];
	let res = match gate.op {
		Op::Move => read(gate.a, gates, wires),
		Op::Not => !read(gate.a, gates, wires),
		Op::And => read(gate.a, gates, wires) & read(gate.b, gates, wires),
		Op::Or => read(gate.a, gates, wires) | read(gate.b, gates, wires),
		Op::LShift => read(gate.a, gates, wires) << read(gate.b, gates, wires),
		Op::RShift => read(gate.a, gates, wires) >> read(gate.b, gates, wires),
	};
	wires.insert(wire, res);
	res
}

pub fn run(input: String) -> Result<()> {
	let mut wires = HashMap::<&str, u16>::new();
	let mut gates = HashMap::<&str, Gate>::new();

	for line in input.lines() {
		let (op, dst) = line.split_once(" -> ").unwrap();
		let op = op.split(' ').collect::<Vec<_>>();
		match op.len() {
			1 => gates.insert(dst, Gate { op: Op::Move, a: op[0], b: "" }),
			2 => gates.insert(dst, Gate { op: Op::Not, a: op[1], b: "" }),
			3 => match op[1] {
				"AND" => gates.insert(dst, Gate { op: Op::And, a: op[0], b: op[2] }),
				"OR" => gates.insert(dst, Gate { op: Op::Or, a: op[0], b: op[2] }),
				"LSHIFT" => gates.insert(dst, Gate { op: Op::LShift, a: op[0], b: op[2] }),
				"RSHIFT" => gates.insert(dst, Gate { op: Op::RShift, a: op[0], b: op[2] }),
				_ => unreachable!(),
			},
			_ => unreachable!(),
		};
	}

	let a = solve("a", &gates, &mut wires);
	println!("part1: {}", a);
	wires.clear();
	wires.insert("b", a);
	println!("part2: {}", solve("a", &gates, &mut wires));
	Ok(())
}
