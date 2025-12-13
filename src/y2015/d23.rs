#[allow(non_camel_case_types)]
enum Op {
	hlf,
	tpl,
	inc,
	jmp,
	jie,
	jio,
}
struct Inst {
	op: Op,
	reg: char,
	offset: i16,
}

fn simulate(insts: &[Inst], a: u64) -> u64 {
	use Op::*;

	let (mut pc, mut a, mut b) = (0i16, a, 0u64);
	while let Some(inst) = insts.get(pc as usize) {
		let reg = if inst.reg == 'a' { &mut a } else { &mut b };
		match inst.op {
			hlf => *reg /= 2,
			tpl => *reg *= 3,
			inc => *reg += 1,
			jmp => pc += inst.offset,
			jie => pc += if *reg % 2 == 0 { inst.offset } else { 0 },
			jio => pc += if *reg == 1 { inst.offset } else { 0 },
		}
		pc += 1;
	}

	b
}

pub fn run(input: String) -> Option<()> {
	use Op::*;

	let mut insts = Vec::new();
	for line in input.trim().lines() {
		let (op, rest) = line.split_once(' ')?;
		let (arg1, arg2) = match rest.split_once(", ") {
			Some((arg1, arg2)) => (arg1, arg2),
			_ => (rest, ""),
		};

		let reg = arg1.chars().next()?;
		insts.push(match op {
			"hlf" => Inst { op: hlf, reg, offset: 0 },
			"tpl" => Inst { op: tpl, reg, offset: 0 },
			"inc" => Inst { op: inc, reg, offset: 0 },
			"jmp" => Inst { op: jmp, reg: '\0', offset: arg1.parse::<i16>().ok()? - 1 },
			"jie" => Inst { op: jie, reg, offset: arg2.parse::<i16>().ok()? - 1 },
			"jio" => Inst { op: jio, reg, offset: arg2.parse::<i16>().ok()? - 1 },
			_ => unreachable!(),
		})
	}

	let b = simulate(&insts, 0);
	println!("part1: {b}");

	let b = simulate(&insts, 1);
	println!("part2: {b}");
	Some(())
}
