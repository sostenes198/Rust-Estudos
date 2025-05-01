# Rust-Estudos

---
# Documentations
[https://doc.rust-lang.org/reference/introduction.html](https://doc.rust-lang.org/reference/introduction.html) <br>
[https://doc.rust-lang.org/nomicon/intro.html](https://doc.rust-lang.org/nomicon/intro.html) <br>
[https://doc.rust-lang.org/unstable-book/library-features/test.html](https://doc.rust-lang.org/unstable-book/library-features/test.html) <br>
**Cargo:** [https://doc.rust-lang.org/stable/cargo/](https://doc.rust-lang.org/stable/cargo/) <br>
**Macros:** [https://doc.rust-lang.org/reference/macros-by-example.html](https://doc.rust-lang.org/reference/macros-by-example.html) <br>
**Book About Rust Macros** [https://veykril.github.io/tlborm/introduction.html](https://veykril.github.io/tlborm/introduction.html)

---
# Cargo
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because 
Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, 
and building those libraries. (We call the libraries that your code needs dependencies.)

## Comandos úteis `cargo`
[https://doc.rust-lang.org/stable/cargo/](https://doc.rust-lang.org/stable/cargo/)

![cargo_folder_1.png](./imgs/cargo_folder_1.png)

![cargo_folder_2.png](./imgs/cargo_folder_2.png)

> `which cargo` Show where cargo is on the machine

> `ls ~/.cargo/bin` List all .exe installed inside folder 

> `cargo --version --verbose` (Show cargo version)

> `cargo doc --open` (command will build documentation provided by all your dependencies locally and open it in your browser.)

> `cargo install {{PACKAGE_NAME}}` Install installed global packages

> `cargo install --list` List installed global packages
 
> `cargo add {{CRATE_NAME}}` Add dependencies to a Cargo.toml manifest file

> `cargo add {{CRATE_NAME}} --dev` Add dev-dependencies to a Cargo.toml manifest file

> `cargo remove {{CRATE_NAME}}` Remove dependencies to a Cargo.toml manifest file

> `cargo remove {{CRATE_NAME}} --dev` Remove dev-dependencies to a Cargo.toml manifest file

> `cargo new` {{PROJECT_NAME}} Create a new binary project using cargo

> `cargo new {{PROJECT_NAME}} --lib ` }} Create a new lib project using cargo

> `cargo check` (Check your coe to make ure it compiles successfully)

> `cargo clean` Remove artifacts that cargo has generated in the past

> `cargo build` (Build entire project)

> `cargo build --release` (Build entire project with optimizations)

> `cargo run` (Build and run project)

> `cargo run -p {{PROJECT_NAME}}` can specify which package in the workspace we want to run by using the -p argument and the package name

> `RUST_BACKTRACE=1 cargo run` Execute rust more details about error handler

> `cargo test` Build and run tests in project

> `cargo test test_name` Runs all the tests that matched  `test_name`

> `cargo test --test file_name` Runs all the tests  in specific rust file  `file_name`

> `cargo test -- --ignored` Runs only tests containing the `ignore` attribute

> `cargo test -- --include-ignored` Run all tests whether they’re ignored or not.

> `cargo test --help` Displays the options you can use with cargo test

> `cargo test -- --help` Displays the options you can use after the separator

> `cargo test -- --test-threads=1` Set the number of test threads to 1, telling the program not to use any parallelism.

> `cargo test -- --show-output` Show printed values for passing tests as well, we can tell Rust to also show the output of successful tests

> `cargo test -p ` Run tests for one particular crate in a workspace from the top-level directory

> `cargo add` {{PACKAGE_NAME}} (Add dependencies from a Cargo.toml manifest file)

> `cargo remove` {{PACKAGE_NAME}} (Remove dependencies from a Cargo.toml manifest file)

> `cargo update` (Update packages inside fix version: example 0.1.1 -> 0.1.9, to change minor and major version, you need to change manually version in Cargo.toml)

> `cargo login {{GENERATED_TOKEN}}` Login into creates.io 

> `cargo logout ` Disconnect from crates.io 
 
> `cargo publish` Publish local package to crates.io 

> `cargo yank --vers {{PACKAGE_VERSION}}` Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue

> `cargo yank --vers {{PACKAGE_VERSION}} --undo` undo a yank and allow projects to start depending on a version again

---
# Prelude
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. 
This set is called the prelude,

[https://doc.rust-lang.org/std/prelude/index.html](https://doc.rust-lang.org/std/prelude/index.html)

# Crates.io
Crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable.

[https://crates.io/](https://crates.io/)
 
Location where cargo credentials are stored: `~/.cargo/credentials.`<br>
File name: `credentials.toml`

---
# Comandos úteis `rustc`

> `rustup show`  Show the active and installed toolchains or profiles

> `rustc --version` (Show current rust version)

> `rustup update` (Update rust)
 
> `rustup self uninstall` (Uninstall rust)
 
> `rustup doc` (Show offline documentation)

> `rustup install {{LIB_NAME}}`

> `rustup uninstall {{LIB_NAME}}`

> `rustup target --help` Show all targets commands

> `rustup target list` Show all targets

> `rustup target list --installed` Show installed targets

> `rustup component --help` Show all components commands 

> `rustup component list --installed` Show all installed components like rustfmt, clippy, etc.. 

> `rustup component list ` Show all available components like rustfmt, clippy, etc.. 

> `rustc {{FILE_MAIN.rs}}` (Build .exe in windows)

![rustup_show_all_installed.png](./imgs/rustup_show_all_installed.png)

![rustup_install_definition.png](./imgs/rustup_install_definition.png)

![rustup_target.png](./imgs/rustup_target.png)

![rustup_component_definition.png](./imgs/rustup_component_definition.png)


---
# Pacótes úteis
[https://crates.io/crates/random](https://crates.io/crates/random) (Pacote para gerar números aleatórios)

---
# Cargo cache
Instalar pacote globalmente `cargo install cargo-cache`

> `cargo cache` Show insalled cargos


---
# Testes


## tarpaulin

Pacote para gerar coverage `tarpaulin` <br>
Instalar pacote globalmente no cargo `cargo install cargo-tarpaulin`

### Comandos

> `cargo tarpaulin --tests` Executa todos os testes usando o pacote `tarpaulin`

> `cargo tarpaulin --tests --out Html` Executa todos os testes gerando resultado em um arquivo **HTML**

> `cargo tarpaulin --tests --test unit_test --out Html` Exemplo para executar somente testes de um arquivo específico
 
> `cargo tarpaulin --tests --test unit_test --out Html --exclude-files "*/mod.rs" --exclude-files "src/main.rs" --exclude-files "tests/*"` Exemplo para executar somente testes de um arquivo específico ignorando outros arquivos/pastas

---

## llvm-cov

Pacote para gerar coverage `llvm-cov` <br>
Instalar pacote globalmente no cargo `cargo install llvm-cov`

> `cargo llvm-cov --html` Executa todos os testes e exibe o coverage na linha de comando

> `cargo llvm-cov --html` Executa todos os testes e exibe o coverage em um arquivo HTML

> `cargo llvm-cov --html --test unit_test  --ignore-filename-regex 'main\.rs$|mod\.rs$'` Exemplo para executar somente testes de um arquivo específico ignorando outros arquivos/pastas

---
# Rust UI
![rust_ui_summary.png](./imgs/rust_ui_summary.png)

1. [egui](https://github.com/emilk/egui)
2. [iced](https://github.com/iced-rs/iced)
3. [druid](https://github.com/linebender/druid)
4. [fltk-rs](https://github.com/fltk-rs/fltk-rs)
5. [slint](https://slint.dev/)
6. [gtk-rs](https://gtk-rs.org/)