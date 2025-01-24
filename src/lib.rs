//! A simple library that provides functionality to prompt for user input.
//! The library is designed to simplify user input handling, offering an easy way to display prompts
//! and capture input without needing to handle `Result` or `Option` explicitly.

use std::io::{self, Write};

/// A trait that defines prompt functionality for obtaining user input as a `String`.
/// 
/// This trait allows any type that implements `StringPromptable` to provide a prompt
/// for user input, making it easier to capture data of type `String`.
/// 
/// # Methods
/// 
/// - `prompt`: Displays a prompt message to the user and returns the input as the type implementing the trait.
pub trait StringPromptable {
    /// Displays a prompt message to the user and returns the input as the type that implements the trait.
    ///
    /// # Parameters
    ///
    /// - `output`: The message that will be shown to the user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let name: String = String::prompt("What is your name? ");
    /// ```
    fn prompt(output: &str) -> Self;
}

impl StringPromptable for String {
    fn prompt(output: &str) -> Self {
        print!("{}", output);
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the prompt");

        input.pop();

        input
    }
}
