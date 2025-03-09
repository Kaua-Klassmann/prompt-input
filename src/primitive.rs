use std::io::{self, Write};

pub trait Promptable {
    fn prompt(output: &str) -> Self;
}

pub trait PromptableWithOption
where
    Self: Sized,
{
    fn prompt(output: &str) -> Option<Self>;
}

pub trait PromptableWithAlternatives
where
    Self: Sized,
{
    fn prompt(output: &str, trues: Vec<&str>, falses: Vec<&str>) -> Option<Self>;
}

impl Promptable for String {
    fn prompt(output: &str) -> Self {
        print!("{}", output);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the prompt");

        input.pop();

        input
    }
}

impl PromptableWithAlternatives for bool {
    fn prompt(output: &str, trues: Vec<&str>, falses: Vec<&str>) -> Option<Self> {
        let input = String::prompt(output).to_lowercase();

        let mapped_trues: Vec<String> = trues.into_iter().map(|w| w.to_lowercase()).collect();
        let mapped_falses: Vec<String> = falses.into_iter().map(|w| w.to_lowercase()).collect();

        if mapped_trues.contains(&input) {
            return Some(true);
        }

        if mapped_falses.contains(&input) {
            return Some(false);
        }

        return None;
    }
}

macro_rules! create_promptable_with_option {
    ($($type:ty), *) => {
        $(
            impl PromptableWithOption for $type {
                fn prompt(output: &str) -> Option<Self> {
                    let input = String::prompt(output);

                    input.parse().ok()
                }
            }
        )*
    };
}

create_promptable_with_option!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64, char
);
