pub enum ResponseStatusType {
    Error(String),
    Success(String),
}

pub const SAMPLE_MD: &str = r#"# Exploring the Wonders of Rust
Welcome to my blog! Today, I'm excited to dive into the world of Rust programming. Whether you're a seasoned developer or just starting out, Rust offers a unique blend of performance and safety that's hard to beat.

## Why Rust?

Rust is a systems programming language that empowers developers to write reliable and efficient software. Its ownership model ensures memory safety without the need for a garbage collector, making it an excellent choice for performance-critical applications.

### Key Features of Rust

1. **Memory Safety**: Rust's unique ownership system prevents null pointer dereferences and buffer overflows.
2. **Concurrency**: Rust makes it easier to write safe and efficient concurrent code.
3. **Performance**: Rust's performance is comparable to C and C++, making it suitable for system-level programming.
4. **Tooling**: Rust comes with integrated tools like Cargo, Clippy, and Rustfmt, which enhance the development experience.

## Getting Started with Rust

To get started with Rust, you'll need to install the Rust toolchain. The easiest way to do this is by using `rustup`, the Rust toolchain installer. You can install it by running the following command in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
"#;