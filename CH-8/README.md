# Common Collections
- Rust standard library contains a number of very useful data structures called `collections`
- Collection can contain multiple values
- Data in these collection is stored in the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
- Three collection that are used very often in Rust\
    1. Vector
    2. String
    3. Hash Map

## Storing Lists of values with vectors
- `Vec<T>` also known as vector.
- Vectors allow you to store more than one value in a single data structure that puts all the value next to each other in memory.
- Vector can only store values of same type

### Creating a New Vector
```rust
let v: Vec<i32> = Vec::new();
```
- `Vec<T>` type provided by the standard library can hold any type.
- Rust provides `vec!` macro. 
```rust
let v = vec![1,2,3];
```

### Updating a Vector
- We can use the push method
```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### Reading Elements of Vector
- Two ways to reference a value stored in a vector
    - via indexing 
    - using the `get` method
```rust
let v = vec![1,2,3,4,5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<i32> = v.get(2);

match third{
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element"),
}
```
- using `&` and `[]` gives us a reference to the element at the index value.
- when we use the `get` method with the index passed as an argument, we get an `Option<&t>` that we can use with `match`
- The reason for having two ways of referencing an element is so we can chose how the program behaves when we try to use an index value outside the range of existing elements.
```rust
let v = vec![1,2,3,4,5];
let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```
- `&v[100]` will cause the program to panic because it references a non existent element.
- This is best used when we want out program to crash if there's an attempt to access an element past the end of the vector.
- with `get` method, if we pass index that is outside the vector, it returns `None` without panicking.
- we would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
- our code will then have logic to handle having either `Some(&element)` or `None`.
- when a program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the content of the vector remain valid.
- Rule: we can't have mutable and immutable references in the same scope.
```rust
let mut v = vec![1,2,3,4,5];
let first = &v[0];
v.push(6);
println!("The first element is: {first}");
```
- here we hold an immutable references to the first element in a vector and try to add an element to the end.
- This program won't work if we also try to refer to that element later in the function.
- this error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating the new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where vector is currently stored.
- in that case, the reference to the first element would be pointing to deallocated memory.
- The borrowing role prevents programs from ending up in that situation.    

### Iterating over the values in a Vector
```rust
let v = vec![100,32,57];
for i in &v{
    println!("{i}");
}
```
- above loop get immutable references to each element in a vector of `i32` values
```rust
let mut v = vec![100,32,57];
for i in &mut v{
    *i += 50;
}
```
- We have to use `*` to dereference to get the valur in `i`.

### Using an Enum to Store Multiple Types
- Vectors can only store values that are the same type.
- If we need to one type to represent elements of different types, we can define and use an enum
```rust
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(1.12),
]
```
- Rust needs to know what types will be in the Vector at compile time so it knows exavtly how much memory on head will be needed to store each element.
- We must also be explicit about what types are allowed in this vector.
- If Rust allowed a vector to hold any type, there would be a change that one or more of the types would cause errors with the operations performed on the elements of the vector.
- Using an enum plus a `match` means that Rust will ensure at compile time that every possible case is handled.

### Dropping a Vector Drops its elements
```rust
{
    let v = vec![1,2,3,4,5];
    // do stuff with v
} // <- v goes out of scope and is freed here.
```