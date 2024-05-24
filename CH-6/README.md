# Enums and Pattern Matching

## Defining an Enum
- Enums give you a way of saying value is one of the possible set of values.
- [Example 1](/CH-6/ex-1.rs)
- [Example 2](/CH-6/ex-2.rs)
- Define an enum
```rust
enum IpAddrKind{
    V4,
    V6,
}
```
- create instances of enum
``` rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
- variants of rnum are namespaces under it's identifies, and we use a double colon to separate the tow.
- we can define a function that takes any IpAddrKind:
``` rust
fn route(ip_kind: IpAddrKind){

}
```
- we can call the function with either variant
```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```
- We can put data directly into each enum variant.
```rust
enum IpAddr{
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```
- We attach data to each variant of the enum directly so there is no need for an extra struct.
- another advantage of using enum over struct: each variant can have different types and amounts of associated data.
```rust
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 0);
let loopback = IpAddr::V6(String::from("::1"));
```
- Another Example:
```rust
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
- Similarity between struct and enum:we are able to define method on enum also.
- example:
```rust
impl Message{
    fn call(&self){
        //method body would be defined here
    }
}
let m = Message::Write(String::from("hello"))
m.call()
```

## The option enum and it's advantages over null values
- `Option` type encodes the very common scenario in which a value could be something or it could be nothing.
- Example: if you request the first item in a non-empty, you would get a value. if you request the first item in an empty list, you would get nothing.
- Rust does not have the null feature that many other languages have.
- the porblem with null values is that if you try to use a null value as a not-null value, you'll get an error of some kind. because this null or not-null rpoperty is pervasive, it's extremly easy to make this kind of error.
- Rust does not have nulls, but it does have an enum that can encode the concept of vaalue being present or absent. This enum is Option<T>
```rust
enum Option<T>{
    None,
    Some(T),
}
```
- it's included in prelude.
- We can use `Some` and `None` directly without the `Option::` prefix.
- `<T>` syntax: generic type parameter. means that `Some` variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type different type.
- Example:
```rust
let some_number = Some(5);
let some_char = Some('e');
let absent_number: Option<i32> = None;
```
- the `some_number` is `Option<i32>`
- the `some_char` is `Option<char>`
- Rust can infer these types because we've specified a value inside the `Some` variant.
- for `absent_number` Rust requires us to annotate the overall `Option` type: the compiler can't infer the type that the corresponding `Some` variant will hold by looking only at a `None` value. here we teell Rusr we mean type of `Option<i32>`
- `Option<T>` and `T` are different types, the compiler wont' let us use an `Option<T>` values as if it were definitely a valid value.
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // this won't compile
```
- we would have to convert an `Option<T>` to `T` before you can perform operation with it.

## the match control flow construct
- `match` allows you to compare a value against a series of pattern and then execute code based on which pattern matches
```rust
enum coin{
    Penny,
    Nickel,
    Dime,
    Quater,
}
fn value_in_cents(coin: Coin){
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>2,
        Coin::Dime=>10,
        Coin::Quater=>25,
    }
}
```
- Difference with `if`: `if` requires the values to be boolean, but here it can be any value.
- `match` arms:
    - An arm has two parts: a pattern and some code
    - `=>` operater separates the pattern and the code
```rust
fn value_in_cents(coin: Coin){
    match coin{
        Coin::Penny=>{
            println!("Lucky penny");
            1
        }
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quater=>25,
    }
}
```
### Patterns that bind to values
```rust
enum UsState{
    Alabama,
    Alaska,
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}
```
- each US state will have a different Design for Quater, we want to store state value inside `Quater`
```rust
fn value_in_cents(coin: Coin){
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quater(state)=>{
            println!("State Quater from {:?}", state),
        },
    }
}
```
### Matching with `Option<T>`
```rust
fn plus_one(x: Option<i32>)-> Option<i32>{
    match x{
        None=> None,
        Some(i)=> Some(i+1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
### Matches are exhaustive
- the arms patterns must cover all possibilitees. Otherwise we'll get error.
```rust
fn plus_one(x:Option<i32>)-> Option<i32>{
    match x {
        Some(i) => Some(i+1),
    }
}
```
- we didn't handle the `None` case, so this  code will cause a bug. It's a bug Rust knows how to catch.
- If we compile this code we will get get error.
- Rust knows that we didn't cover every possible case, an even knows which pattern we forgot.
- Matches in Rust are exhaustive, we must exhaust every last possibility in order for the code to be valid.

### Catch-all Patterns and the _ Placeholder
- we can also take special actions for a few particular value, but for all other values take one default action.
```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_spaces: u8){}
```
- the last pattern will match all values not specifically listed. 
- this catch-all pattern meets the requirement that `match` must be exhaustive.
- Rust also has a pattern we can use when we want a catch-all but don't want to use the value in the catch-all pattern: `_` is a special pattern that matches any values and does not bind to that value.
- this tells rust we aren't going to use the value, so Rust won't warn us about an unused variable.
```rust
let dice_roll = 9;
match dice_roll{
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn reroll(){}
```

## Concise Control flow with `if let`
- combines `if`