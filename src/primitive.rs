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

impl PromptableWithOption for i32 {
    fn prompt(output: &str) -> Option<Self> {
        let input = String::prompt(output);

        input.parse().ok()
    }
}
