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
- To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a file system.
- A path can take two forms:
    - An absolute path: full path starting from crate root. it starts with the literal `crate`
    - A relative path: starts from the current module and uses `self`, `super` or an identifier in the current module.
- example:
    - calling `add_to_waitlist` function from a new function `eat_at_restaurant` defined in the crate root.
```rust
mod front_of_house{
    mod hosting{
        fn add_to_waitlist(){}
    }
}
pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
- How to chose whether to use a relative or absolute path.
    - depends on whether you're more likely to move item definition code separately from or together with the code that uses the item.
    - If we move the whole module we need to update absolute path but relative path would be valid.
    - if we move the function to separate module, the path call would stay the same, but the relative path would need to be updated.
- Preference: in general to specify absolute paths.
- If we compile the above example, it will fail
    - The error message say that module `hosting` is private.
    - Even if we have correct paths for our functions, Rust won't let us use them because it doesn't have access to the private sections.
    - In Rust all items are private to parent modules by default.
    - If we want to make an item like a function or struct `private` you put it in a module.
    - Items in parent can't use the private items inside child modules, but items in child modules can use the item in their ancestor modules.
    - This is becuase child modules wrap and hide their implementation details, but the child modules can see the context in which they're defines. 
    - Rust does give you the option to expose inner parts of child modules code to outer ancestor module by using the `pub` keyword to make an item public.
### Exposing Paths with the `pub` keyword
- updating the above example
```rust
mod front_of_house{
    pub mod hosting{
        fn add_to_waitlist(){}
    }
}
pub fn eat_at_restaurant(){
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}
```
- still fails to compile
- Adding the pub keyword infront of `mod hosting` makes the module public.
- With this change, if we can access `front_of_house`, we can access `hosting`. 
- But contents of hosting are still private, making the module public doesn't make it's content public.
- the `pub` keyword on a module only lets code in it's ancestor module refer to it, not access it's inner code. 
- need to make `add_to_waitlist` public
```rust
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
pub fn eat_at_restaurant(){
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}
```
- while `front_of_house` isn't public, because the `eat_at_restaurant` function is defined in the same module as `front_of_house` from `eat_at_restaurant`, so the relative path starting from the module in which `eat_at_restaurant`is defined works.

### Starting Relative paths with `super`
- We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path.
- Example:
```rust
fn deliver_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){};
}
```

### Making Structs and Enums public
- We can use `pub` to designate structs and enums public, but if we use `pub` before struct definition, we make the struct public, but the struct's field will still be private. 
- we can make each field public or not on a case-to-case basis
- Example
```rust
mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant(){
    // Order a breakfast in the summer with Rye Toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed to see or modify the seasonal fruit that comes
    // with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```
- we cannot use the `seasonal_fruit` field in `eat_at_restaurant` because `seasonal_fruit` is private.
- `back_of_house::Breakfast` has a private field, the struct needs to provide a public associated function that constructs an instance of `Breakfast`
- If we make an enum public, all of it's variants are then public.
- Example
```rust
mod back_of_house{
    pub enum Appetizer{
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant(){
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## Bringing paths into scope with the `use` keyword
- Whether we chose the absolute or relative path, every time we have specify some path.
- We can create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope.
- Example:
```rust
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
use crate::front_of_house:::hosting
pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}
```
- `use` only created the shortcut for the particular scope in which `use` occurs.
```rust
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
use crate::front_of_house::hosting;
mod customer{
    pub fn eat_at_restaurant(){
        hosting::add_to_waitlist();
    }
}
```
- Above example will fail to compile, since shortcut no longer applied within `customer` module
- to fix this
    - move the `use` within the `customer` module
    - reference the shortcut in the parent module with `super::hosting` within the child `customer` module.

### Creating Idiomatic use Paths
- We bring the function's parent module into scope with `use`, means we have to specify parent modules when calling the function.
- Specifying the parents module when calling the function makes it clear that the function isn't locally defined while minimizing repetetion of full path.
```rust
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant(){
    add_to_waitlist();
}
```
- Here it's unclear as to where `add_to_waitlist` is defined.
- When bringing in stucts, enums and other items with `use`, it's idiomatic to specify the full path.
```rust
use std::collections::HashMap;

fn main(){
    let mut map = HashMap::new();
    map.insert(1,2);
}
```
- this is just a convention.
- if we're bringing two items with the same name into scope with `use` statements, Rust won't allow that.
```rust
use std::fmt;
use std::io;

fn function1()->fmt::Result{
    //
}
fn function2()->io::Result{
    // 
}
```
### Providing new names with the `as` keyword
- We can specify `as` and a new local name or `alias` for the tupe.
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1()->Result{
    //
}
fn function2()->IoResult{
    
}
```
### Re-exporting names with `pub use`
- When we bring a name into scope with the `use` keyword, the name avialable in the new scope is private.
- to enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine `pub` and `use`.
- This is called re-exporting.
```rust
// restaurant.rs
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}
```
- Before this change, we would call add_to_waitlist function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`
- with the use of `pub use` we can use the path `restaurant::hosting::add_to_waitlist()`

### Using External Packages
- Take example for using `rand`
- Steps to use `rand`
    1. add rand in `cargo.toml` as `rand = "0.8.5"`
    2. Bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the crate and listed the items we wanted to bring into scope.
    ```rust
    use rand::Rng;
    fn main(){
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }
    ```
- `std` library is also a crate external to our package. But it's shipped with Rust language, we don't need to change `cargo.toml` to include `std`

### Using Nested Paths to clean up Large `use` lists
- when importing multiple items defined in the same crate or same module, listing each item on it's own line can take too much space.
```rust
use std::cmp::ordering;
use std::io;
```
- We can use nested paths to bting same items into scope in one line
```rust
use std::{cmp::ordering, io};
```
- this can reduce number of separate `use` statements
```rust
use std::io;
use std::io::write;
```
- here common path is `std::io`, to merge these two paths, we can use `self` in the nested path;
```rust
use std::io::{self, write};
```

### The Glob Operator
- If we want to bring `all` public items in a path into scope, we can specify that path followed by the `*` glob operator
```rust
use std::collections::*;
```
- Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
- The glob operator is ofter used when testing to bring everything under tests into `tests` module.

## Separating Modules into different files
-