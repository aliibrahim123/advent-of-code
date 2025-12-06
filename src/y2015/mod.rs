use anyhow::Result;

mod d1;
mod d10;
mod d11;
mod d12;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

pub fn run(day: u8, input: String) -> Result<()> {
	match day {
		1 => d1::run(input),
		2 => d2::run(input),
		3 => d3::run(input),
		4 => d4::run(input),
		5 => d5::run(input),
		6 => d6::run(input),
		7 => d7::run(input),
		8 => d8::run(input),
		9 => d9::run(input),
		10 => d10::run(input),
		11 => d11::run(input),
		12 => d12::run(input),
		_ => todo!(),
	}
}
