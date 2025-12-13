use anyhow::Result;
use num_traits::Num;

pub trait OptionExt<T> {
	fn ok(self) -> Result<T>;
	fn discard(self) -> ();
}
impl<T> OptionExt<T> for Option<T> {
	fn ok(self) -> Result<T> {
		self.ok_or(anyhow::anyhow!("Unexpected None"))
	}
	fn discard(self) -> () {
		()
	}
}

pub trait StrExt {
	fn first_char(&self) -> char;
}
impl StrExt for str {
	fn first_char(&self) -> char {
		self.chars().next().unwrap()
	}
}

pub fn div_ciel<T: Num + Copy>(a: T, b: T) -> T {
	(a + b - T::one()) / b
}
