pub mod utils;
pub mod y2015;

#[macro_export]
macro_rules! year_mod {
    ($($day:literal),*) => {
        paste::paste! {
            $( mod [<d $day>]; )*
            pub fn run(day: u8, input: String) -> Option<()> {
                match day {
                    $( $day => [<d $day>]::run(input), )*
                    _ => None,
                }
            }
        }
    };
}
