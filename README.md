# Rust-Estudos

---
# Cargo
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because 
Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, 
and building those libraries. (We call the libraries that your code needs dependencies.)

## Comandos úteis `cargo`
[https://doc.rust-lang.org/stable/cargo/](https://doc.rust-lang.org/stable/cargo/)

1. `cargo --version` (Show cargo version)
2. `cargo doc --open` (command will build documentation provided by all your dependencies locally and open it in your browser.)
3. `cargo new` {{PROJECT_NAME}} (Create a new directory and project using cargo)
4. `cargo check` (Check your coe to make ure it compiles successfully)
5. `cargo build` (Build entire project)
6. `cargo build --release` (Build entire project with optimizations)
7. `cargo run` (Build and run project)
8. `cargo add` {{PACKAGE_NAME}} (Add dependencies from a Cargo.toml manifest file)
9. `cargo remove` {{PACKAGE_NAME}} (Remove dependencies from a Cargo.toml manifest file)
10. `cargo update` (Update packages inside fix version: example 0.1.1 -> 0.1.9, to change minor and major version, you need to change manually version in Cargo.toml)

---
# Prelude
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. 
This set is called the prelude,

[https://doc.rust-lang.org/std/prelude/index.html](https://doc.rust-lang.org/std/prelude/index.html)

# Crates.io
Crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable.

[https://crates.io/](https://crates.io/)

---
# Comandos úteis `rustc`

1. rustc --version (Show current rust version)
2. rustup update (Update rust)
3. rustup self uninstall (Uninstall rust)
4. rustup doc (Show offline documentation)
5. rustc {{FILE_MAIN.rs}} (Build .exe in windows)


---
# Pacótes úteis
[https://crates.io/crates/random](https://crates.io/crates/random) (Pacote para gerar números aleatórios)