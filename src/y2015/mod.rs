use anyhow::Result;

mod d1;

pub fn run(day: u8, input: String) -> Result<()> {
	match day {
		1 => d1::run(input),
		_ => todo!(),
	}
}
