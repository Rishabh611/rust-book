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

## An example program using Structs
- [Example](/CH-5/ex-9-area-of-rectangle/src/main.rs)
- Structs add more meaning to our program
```
let rec1 = Rectangle{
    width: 30,
    height: 40,
};
println!("rect1 is {}", rect1);
```
- this will thow error. 
- `error[E0277]: `Rectangle` doesn't implement `std::fmt::Display``
- Primitives implement Display by default because there is only one way to show them
- But with structs, the way `prinln!()` should format the output is less clear because there are more display possibilities.
- Rust does not want to guess what we want, and structs don't have a provided implementation of `Display` to use with `println!()` and the {} placeholder
- we can change the println to `println!("rect1 is {:?}")`.
- putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.
- We still get an error. We need to explicitly opt in to make the Debugging functionality available to us.
- we need to add the outer attribute `#[derive(Debug)]` just before struct definition.
- when we have large struct, it's useful to have output that's a bit easier to read. In those case we can use `{:#?}`
- Another way to print out a value using `Debug` format is to use the `dbg!` macro which takes ownership of an expression, prints the file aand line number whre that `dbg!` macro call occurs in our code along wit the resultant value of that expression.

## Method Syntax
- Method are similar to functions :we declare them with the `fn` keyword and a name, they can have parameters and a return value.

### Defining Methods
```
imply Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    };
}
```
- We start with `impl` block for `Rectangle`. 
- Every thing within `impl` block will be associated with the `Rectangle` type. 
- We change the first paramerter to be `self` in the sifnature and everywhere in the body. 
- We use `&self` instead for `rectangle: &Rectangle`. 
- The `&self` is actually short for `self:&self`. 
- within impl block, the type `Self` is an alear for the type that the impl block is for.
- Method must have a parameter named `self` of type `Self` for their first parameter. So Rust lets you abbreviate this with only the name `self`.
- We used `&self` because we don't want to take ownership and we just want to read data in the struct not write to it.
- if we wanted to change the instance that we've called the method as part of what the method does, we'd use `&mut self` as the first parameter.
- main reason for using method instead for function is for organization.

### Method with more parameters
- Method to compare a rectangle with another
```
impl Rectangle{
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
}
```

### Associated functions
- All function defines within a `impl` block are called `Associated function` because they're associated with the type name after the `impl`.
- we can define associated function that don't have `self` as their first parameter because they don't need an instance of the type to work with.
- Associated function that aren't methods are used often for constructors that will return a new instance of the struct. 
```
impl Rectangle{
    fn square(size: u32)->Self{
        Self {
            width: size,
            height: size,
        }
    }
}
```
- The Self keyword in thereturn type and in the body of the function are aliases for the type that appears after the `impl` keyword.
- To call these function, we use the `::` syntax with the struct. 
- The `::` syntax is used for both associated functions and namespaces created by modules.

