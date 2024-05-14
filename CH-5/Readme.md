# Using structs to structure related data

- A `struct`, short for structure, is a custom data type in Rust. 
- It allows you to group multiple related values together under a single name, forming a meaningful group of data. 
- You can think of a `struct` as similar to an object's data attributes.

## Defining and instantiating Structs

- Structs are similar to tuples, but with a key difference: you name each piece of data, making it clear what the values represent. Here's an [example](/CH-5/ex-1.rs).

- Inside curly braces, we define the names and types of the pieces of data, which are called fields.
- To use a struct, we create an instance of that struct by specifying concrete values for each field. See [example](/CH-5/ex-2.rs).

- To access a specific value from a struct, we use dot notation.
- If the instance is mutable, we can change a value by using dot notation and assigning into a particular field. See [example](/CH-5/ex-3.rs).
- Note that the entire instance must be mutable; Rust doesn't allow marking only certain fields as mutable.

- We can construct a new instance of the struct as the last expression in the body to implicitly return that new instance. See [example](/CH-5/ex-4.rs).

### Using the Field init shorthand

- If the parameter names and the struct field names are exactly the same, we can use the field init shorthand syntax to rewrite the instance. See [example](/CH-5/ex-5.rs).

### Creating from another instance using Struct Update Syntax

- It's often useful to create a new instance of a struct that includes most of the values from another instance but with some changes.
- The syntax `..` specifies that the remaining fields not explicitly set should have the same values as the fields in the given instance. See [example](/CH-5/ex-6.rs).
- Note that `..user1` should come last to specify that any remaining fields should get their value from the corresponding fields in `user1`.

### Using Tuple Structs without named fields to create different types

- Rust supports `tuple structs`, which look similar to tuples but have the added meaning of the struct name.
- However, they don't have names associated with their fields; rather, they just have the types of the fields.
- Tuple structs are useful when you want to give the whole tuple a name and make it a different type from other tuples, especially when naming each field would be verbose or redundant. See [example](/CH-5/ex-7.rs).
- Each struct we define is its own type, even though the fields within the struct might have the same type.

### Unit-Like Struct without any fields

- We can define structs that don't have any fields, called `unit-like` structs because they behave similarly to `()`. 
- Unit-like structs can be useful when we need to implement a trait on some type but don't have any data that we want to store in the type itself. See [example](/CH-5/ex-8.rs).


### Ownership of Struct Data

Check later.