# Managing Growing Project with Packages, Crates, and Modules
- As a project grows, you should organize code by splitting it into multiple modules and then multiple files.
- A package can contain multiple binary creates and optionally one library creat.
- As a package grows, you can extract pars into separate creates that become external dependecies.
- Rust has a number of features that allow you to manage your code's organization, including which details are exposed, which details are private and what names are in each scope in your program.
- These features, sometimes collectively reffered as the module system, include:
    - Package: A cargo feature taht lets you build, test and share crates.
    - Crates: A tree of modules that produce a library or executable.
    - Modules and Use: lets you control the organization, scope, and privacy of the path.
    - Paths: A way of naming an item, such as struct, function, or module

## Packages and Crates
- A `crate` is the smallest amount of code that a Rust compiler considers at a time.
- Crates can contain modules, and the modules may be defines in other files that get compiled with the crate.
- A Crate can come in one for two forms:
    - A binary crate
    - a library crate.
- `Binary crates` are programs you can compile to an executable that you can run, such as a command-line program or a server.
- `Library Crates` don't have `main` function, and they don't compile to an executable.
    - instead they define functionality intended to be shared with multiple projects.
- The `create root` is a  source file that the Rust compiler starts from and makes up the root module of your crate.
- A `package` is a bundle of one or more crates that provide a set of functionality.
- A package contains a `Cargo.toml` file that describes how to build those crates.
- Cargo is actually a package that contains the binary crate for the command-line  tool you've  been using to build your code.
- The Cargo package also containes a library crate that the binary crate depends upon.
- A package can contain as many as binary crated as you like, but at most only one library crate.
- A package must contain at least one crate, whether that's a library or binary crate.
- What happens when we run `cargo new`:
    - There's a `cargo.toml` file, giving us a package.
    - There's also a `src` directory that contains main.rs.
    - in `Cargo.toml` there no mention of `src/main.rs`.
    - Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.
    - Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package and `src/lib.rs` is its crate root.
    - Cargo passes the crate root files to `rustc` to build the library or binary

## Defining Modules to Control Scope and Privacy
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
### Modules Cheat Sheet
- Start from the crate root
    - while compiling a crate, the compiler first looks in the crate root file (`src/lib.rs` or `src/main.rs`) for code to compile.
- Declaring Modules: 
    - in crate root file, we can declare new modules. 
    - ex: if we want to declare "garden" module with `mod garden`.
    - The compiler will look for the module's code in these places
        - inline, within curly brackets that replace the semicolon following `mod garden`
        - in the file `src/garden/vegetale.rs`
        - in the file `src/garden/vegetables/mod.rs`
- Declaring submodules:
    - in any file other thatn the crate roor, you can declare submodules.
    - we can declare using `mod vegetables` in `src/garden.rs` . The compiler will look for the submodule's code within the direcoty named for the parent module in these places:
        - inline, directly following `mod vegetables` within curyl brackets instead of the semicolon
        - in the file `src/garden/vegetables.rs`
        - in the file `src/garden/mod.rs`
- Paths to code in modules:
    - once a module is part of our code, we can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. 
    - Example: an `Asparaghus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparahus`.
-Private vs Public: 
    - Code within a module is private from its parent modules by default. 
    - To make a module public, declate it with `pub mod` instead of `mod`.
    - To make items within a public module public as well, use `pub` before their declarations
- The `use` Keyword:
    - Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.
    - In any scope that can refer to `crate::garden::vegetable::Asparaghus`, you can create a shortcut with `use crate::garden::vegetable::Asparaghus` and then from then on we only need to write `Asparagus` to make use of that type in the scope.

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
- The crate root file in this case in `src/main.rs` and it contains:
```rust
use crate::garden::vegetable::Asparagus;
pub mod garden;
fn main(){
    let plant = Asparagus{};
    println!("I'm growing {:?}!",plant);
}
```
- The `pub mod garden` line tells the compiler to include the code it finds in `src/garden.rs`
```rust
pub mod vegetables;
```
- Here, `pub mod vegetables;` means the code in `src/garden/vegetable.rs` is included too.

### Grouping Related Code in Modules
- modules allow us to control the privacy of items, because code within a module is private by default.
- Example ofr a library crate which provides a functionality of a restaurant.
- in restaurant some parts of the a restaurant are reffered to as `front of house` and other as `back of house`.
- Front of house is where customers are, back of the house is where chefs and cooks work in the kitchen etc.
```rust
cargo new restaurant --lib
```
- creates a new library named `restaurant`
- front of the house
```rust
mod front_of_house{
    mod hosting{
        fn add_to_waitlist(){}

        fn seat_at_table(){}
    }
    mod serving{
        fn take_order(){}

        fn serve_order{}

        fn take_payment{}
    }
}
```
- we define a module with the name `mod` keyword followed by the name of the module.
- inside modules, we can place other modules. 
- by using modules, we can group related definitions together and name why they're related.
- crate root:  `src/main.rs` and `src/lib.rs` are called crate roots. the contents of either of these two files form a module named `crate` at the root of the create's module structure known as thee module tree.
```
 crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
- this tree shows how some of the modules nest inside one another. 
- if module A is inside of module B we say that the module A is the child of module B.

## Paths for Reffering to an item in the module tree.
-