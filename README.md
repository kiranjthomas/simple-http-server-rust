# simple-http-server-rust

I wanted to be able to teach others how a HTTP server works under the hood. I wanted to learn a new language and decided to use Rust to facilitate building this guide.

I used the following resources

* <https://doc.rust-lang.org/book/ch20-01-single-threaded.html>
* <https://www.youtube.com/watch?v=7GBlCinu9yg>

## Rust Basics

### Tools

#### Rustup

The Rust toolchain installer. You can use `rustup` to manage different versions of rust.

If you are familiar with python, `rustup` feels equivalent to `pyenv`.

#### Cargo

> Cargo is Rust's build system and package manager.

<https://doc.rust-lang.org/book/ch01-03-hello-cargo.html>

### Commands

<https://www.rust-lang.org/learn/get-starte>

#### Compile

```sh
rustc src/main.rs
```

This will generate an executable file called main. You can run the file via `./main`.

#### Build

```sh
cargo build
```

> This command creates an executable file in target/debug/[whatever].

<https://doc.rust-lang.org/book/ch01-03-hello-cargo.html>

#### Run

Compile the code AND run the resulting executable all in one command

```sh
cargo run
```
