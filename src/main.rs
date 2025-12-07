use std::fs::{self, read_to_string};

use advent_of_code::y2015;
use anyhow::Result;
use clap::Parser;

/// run of code soluion
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	year: u8,
	#[arg(short, long)]
	day: u8,
}

fn main() -> Result<()> {
	let Args { year, day } = Args::parse();

	let input_path = format!("./.temp/inp{year}d{day}.txt");
	let input = if let Ok(input) = read_to_string(&input_path) {
		input
	} else {
		let url = format!("https://adventofcode.com/20{year}/day/{day}/input");
		println!("fetching {url}");
		let token = read_to_string("./.temp/token")?;
		let req = ureq::get(url).header("cookie", format!("session={token}"));
		let res = req.call()?.body_mut().read_to_string()?;
		fs::write(&input_path, &res)?;
		res
	};

	match year {
		15 => y2015::run(day, input).ok_or(anyhow::anyhow!("Unexpected None"))?,
		_ => todo!(),
	};

	Ok(())
}
