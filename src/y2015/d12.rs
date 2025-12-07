use serde_json::Value;

fn match_nb(char: char) -> bool {
	matches!(char, '-' | '+' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')
}

fn count_in(value: &Value) -> i64 {
	match value {
		Value::Number(nb) => nb.as_i64().unwrap(),
		Value::Array(arr) => arr.iter().map(count_in).sum(),
		Value::Object(map) => {
			if map.values().any(|v| *v == "red") {
				return 0;
			};
			return map.values().map(count_in).sum();
		}
		_ => 0,
	}
}

pub fn run(input: String) -> Option<()> {
	let mut sum = 0;

	for nb in input.split(|c| !match_nb(c)).filter(|nb| !nb.is_empty()) {
		sum += nb.parse::<i32>().ok()?;
	}
	println!("part1: {sum}");

	let root = serde_json::from_str::<'_, Value>(&input).ok()?;
	println!("part2: {}", count_in(&root));
	Some(())
}
