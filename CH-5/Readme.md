# Using structs to Structure related data
- `struct` or `structure` => custom data type => package together and name multiple related values that make a meaningful group
-  `struct` is like an object's data attributes

## Defining and instantiating Structs
- Similar to  tuples
- unlike with tuples, in a struct you'll name each piece of  data so it's clear what the values mean
- [Example](/ex-1.rs)
- inside curly brackers, we define names and types of the pieces of data which we calll fields
- to use struct, we create an `instance` of that struct by specigiying concrete values for each of the fields.
- Create an instance of struct [example](/ex-2.rs)
- to get specific value from struct we use dot notation.
- if the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. [Example](/ex-3.rs)
- entrire instance must be mutable. Rust doesn't allow us to mark only certain fields as mutable
- we can construct a new instance of teh struct as the last expression in the body to implicitly return that new instance. [example](/ex-4.rs)
### Using the Field init shorthand
- because the parameter names and the struct field names are exactly the same, we can use the `field init shorthand` syntax to rewrite the instance. [Example](/ex-5.rs)
### Creating from other instance using Struct Update Syntax
- often useful to create a new instance of a struct that includes most of the values from another instance, but some changes
- 