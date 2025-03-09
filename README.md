# prompt-input

A simple and lightweight Rust library for creating user input prompts. The library is designed to simplify input handling in CLI applications, providing a straightforward API to prompt users for input.

## Installation

Add `prompt-input` to your `Cargo.toml`:

```
cargo add prompt-input
```

## Usage

Here's a quick example of how to use `prompt-input`:

```rust
use prompt_input::prelude::*;

fn main() {
    let name: String = String::prompt("Enter your name: ");
    let age: Option<u8> = u8::prompt("Enter your age: ");

    if age.is_none() {
        println!("Hello {}!", name);
    } else {
        println!("Hello {}! You are {} years old.", name, age.unwrap());
    }
}
```

## Why Use prompt-input?

- Reduces boilerplate code for handling user input.
- Encourages clean and readable CLI application code.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/Kaua-Klassmann/prompt-input).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
