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
- The `create root` is a  source file that the Rust comiler starts from and makes up the root module of your crate.
- A `package` is a bundle of one or more crated that provide a set of functionality.
- A package contains a `Cargo.toml` file that describes how to build those crates.
- Cargo is actually a package that contains the binary crate for the command-line  tool you've  been using to build your code.
- The Cargo package also containes a library crate that the binary crate depends upon.
- 