# Getting Started

## Installation
- `rustup`

### Updating
- `rustup update`

### Local Documentation
- `rustup doc`

## Hello, world

### Writing and Running a Rust Program
- rust file always end with `.rs` extenstion.
```rust
fn main(){
    println!("Hello, world!");
}
```
- use `rustc main.rs` to compile the file
- `./main` to run the file.
### Anatomy of a rust program
```rust
fn main(){

}
```
- `main` function is special: it's always the first code that runs in every executable in Rust program.
```rust
println!("Hello, world!");
```
- Four important details to notice
    1. Rust style is to indent with four spaces not a tab
    2. `println!` calls a rust macro. if it had called a function instead it would be entered as `println`
    3. `Hello, world!` String 
    4. We end the line with a semi colon, which indicated that this expression is over and the next one is ready to bigin.

### Compiling and Running are separate steps
- Before running a rust program we must compile it using the Rust compiler by entering `rustc` command and passing it the name of the source file.
- After compiling successfully, Rust outputs a binary executable.
- Rust is `ahead-of-time` compiled langauge. meaning you can compile a program adn give the executable to someone else, and they can run it even without having rust installed.

## Hello, Cargo
- Cargo is Rust's build system and package manager. 
- handles tasks like
    - building your code
    - dounloading the libraries your code depends on
    - building those libraries
- Cargo comes intsalled with rust.
- to check cargo version `cargo --version`
- create a new cargo project `cargo new hello_cargo`
- Cargo generates two files and one folder
    1. `Cargo.toml`
    2. src directory
    3. `main.rs`
- it also initialized a new git repository with a `.gitignore` file.
- Cargo.toml
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```
- `[package]` section: congiguring a package.
- `[dependecies]`: to list any of our projects' dependencies.
- Cargo expects your source files to live inside the src directory.
- the top level direcoty is just for README files, license information, configuration files etc.

### Building and Running a Cargo project
- build your project by entering `cargo build`
- this command creates an executable file in `target/debug/hello_cargo` rather than your current directory. 
- default build is a debug build, cargo puts the binary in a directory named debug.
- Running `cargo build` for the first time also causes Cargo to create a new file `Cargo.lock`
- this file keeps track of the exact version of dependencies in the project.
- cargo manages it's contents for us.
- We can also use `cargo run` to compile the code and then run the resultant excutable all in one command.
- this is more convinient.
- cargo also provies a command called `cargo check`. This command quickly checks your code to make sure it compiles but does not produce an executable.
- `cargo check` is much fater than `cargo build` because it skips the step of producing  an executable.
### Builiding for release
- When a project is finall ready for release, you can use `cargo build --release` to compile it with optimization.
- this command will cretae an executable in target/release.
- the optimizations make your Rust code run faster, but lengthens the time it takes for your program takes to compile.

### Cargo as a Convention
- 