# Generics Types, Traits, and Lifetimes
- `generics`: abstract stand-ins for concrete types or other properties.

## Generics data types
- We use generics to create definations for items like function signatures or structs, which we can then use with many different concrete type.

### in function definations
```rust
fn main() {
    let number_list = vec![43, 50, 234, 134];
    let result = largest_i32(&number_list);
    println!("{result}");
    
    let char_list = vec!['y', 'v', 'c', 'e'];
    let result = largest_char(&char_list);
    
    println!("{result}");
}
fn largest_i32(list: &[i32]) -> &i32{
    let mut largest = &list[0];
    
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}
fn largest_char(list:&[char]) -> &char{
    let mut largest = &list[0];
    for item in list{
        if item>largest{
            largest = item;
        }
    }
    largest
}
```
- We can eliminate duplication by introducing a generic type parameter in a single function
- to parameterize the types in new function, we need to name the type parameter. We can use any identifier but we'll use `T` by convention.
- When we use a type parameter name in the function signature, we have to declare the type parameter name before we use it.
- To define the generic `largest` function, place type name declarations inside angle brackets `<>`, between the name of the function and the parameter list
```rust
fn largest<T> (list: &[T]) -> &T{

}
```
- we read this as: the function `largest` is generic over some type `T`. This function has one parameter named `list`, which is a slice of values of type `T`. The `largest` function will return a reference to a value of the same type `T`
```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}
```
- This will fail
- The states that the body of `largest` won't work for all possible types that `T` could be.
- Because we want to compare values of type `T` in the body, we can only use types whose values can be ordered.
- To enable conparisons, the standard library hs the `std::cmp::PartialOrd` trait that we can implement of types.
```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if item>largest{
            largest = item;
        }
    }
    largest
}
```
### In Struct Definitions
- We can also define structs to use a generic type parameter in one or more fields using the `<>`
```rust
struct Point<T>{
    x: T,
    y: T,
}
fn main(){
    let integer = Point{ x:5, y:10};
    let float = Point{ x: 1.0, y: 4.0};
}
```
- First we declare the name of the type parameter inside angle brackets just after the name of the struct
- Then we use the generic type in the struct definition where we would other specify concrete data types.
- If we create an instance of `Point<T> that has values of different types, our con't won't compile.
- To define a `Point` struct where `x` and `y` both could have different types, we can use multiple generic type parameters
```rust
struct Point<T, U> {
    x: T,
    y: U,
}
fn main(){
    let both_integer = Point{ x: 5, y: 10 };
    let both_float = Point{ x: 1.0, y: 4.0 };
    let integer_and_float = Point { x:5, y: 4.0 };
}
```

### In Enum Definitions
- We can define enums to hold generic data types in their variants
```rust
enum Option<T> {
    Some<T>,
    None,
}
```
- The `Option<T>` enum is generic over type `T` and has two variants: `Some`, which holds one value of type `T` and a `None` variant that doesn't hold any value. 
- By using the `Option<T>` enum, we can express the abstract concept of an optional value, and because `Option<T>` is generic, we can use this abstraction no matter what the type of the option value is.
- Enums can use multiple generic types as well, 
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- The `Result` enum is generic over two types, `T` and `E` and has two variants: `Ok` which holds a value of type `T` and `Err` which holds a value of type `E`. 
- We can use `Result` enum anywhere we have an operation that might succeed or fail. 

### In Method definitions
- We can implement method on stucts and enums and use generic types in their definition too.
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self) -> &T {
        &self.x
    }
}
fn main(){
    let p = Point{ x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```
- We've defined a method named `x` on `Point<T>` that returns a reference to the data in the field `x`
- We can specify constraints on generic types when defining method on the type, 
- We can also implement method only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type.
```rust
impl Point<f32>{
    fn distance_from_origin(&self) -> f32 {
        (self,x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
- This code means the type `Point<f32>` will have a `distance_from_origin` method; other instances of `Point<T>` where `T` is not of type `f32` will not have this method defined.

### Performance of Code using Generics
- Using generics types won't make your program run any slower than it would with concrete types
- Rust accomplishes this by performing monomorphization of the code using generics at compile time.
- `Monomorphization  is the process of turning generics code into specfic code by filling in the concrete types that area used when compiled.
- In this process, compiler does the opposite of the steps we used to create the generic function; the compiler looks att all the places where generic code is called and generates code for the concrete types the generic code is called with.
- Example
```rust
let integer = Some(5);
let float = Some(5.0);
```
- When Rust compiles this code, it performs monomorphization. In that the compiler reads the values that have been used in `Option<T>` instances and identifies two kinds of `Option<T.`: one is `i32` and other is `f64` and replaces the generic defination with the specific one.
```rust
enum Option_i32{
    Some(i32),
    None,
}
enum Option_f64{
    Some(f64),
    None,
}
fn main(){
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
## Traits: Defining shared behaviour
- A `trait` defines a functionality a particular type  has and can share with other types.
- We can use traits to define shared behaviour in an abstract way.
- We can use `trait bound` to specify that a generic type can be any type that has certain behaviour.
- Similar to  `interfaces`.

### Defining a Trait
- A type's behaviour consists of the methods we can call on that type.
- Different types share the same behaviour if we can call the same methods on all of those types.
- Trait definitions are a way to group method signatures together to define a set of behaviour necessary to accomplish some purpose.
- Example:
    - We have multiple structs that hold various kinds and amounts of text.
        - `NewsArticle` in a particular location
        -  `Tweet`: has 280 characters; with metadata that indicate whether it was a new tweet, retweet or reply
    - We want to make a media aggregator library crate named `aggregator` that can display summaries of data. 
    - We need summary from each type, and we'll request that summary by calling `summarize` method on an instance.
    ```rust
    pub trait Summary{
        fn summarize(&self) -> String;
    }
    ```
    - we declare a trait using `trait` keyword and then the trait's name.
    - we also declared the trait as `pub`. 
    - inside curly brackets, we declare the method signatures that describe the behaviours of the types that implement this trait.
    - instead of providing implementation, we use a semicolon.
    - Each type implementing this trait must provide its own custom behaviour for the body of the method..
    - the compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined.
- A trait can have multiple method in its body.
### Implementing a Trait on a type
- Example
```rust
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format(
            "{}, by {} ({})", self.headline, self.author, self.location
        )
    }
}

pub struct Tweet{
    pub username: String, 
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
- implementing a trait on a type is similar to implementing regular method.
- The difference is that after `impl` we put the trait name we want to implement, the use the `for` keyword, and then specify the name of the type we want to implement the trait for.
- Within the `impl` block we put the method signature that the trait definition has defined and we fill in the method body with the specific behaviour we want the method of the ttrain to have for the particular type.
```rust
use aggregator::{Summary, Tweet};

fn main(){
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println("1 new tweet: {}", tweet.summarize());
}
```
- Other crates that depend on the  `aggregator` crate can also bring `Summary` trait into scope to implement `Summary` on their types.
- One restriction to note is that we can implement a trait on a type only if atleast one of the trait or the type is local to our crate.
- Example
    - We can implement standard library traits like `Display` on a custom type like `Tweet` as part of our aggregator crtae functionality
    - because the type `Tweet` is local to our `aggregator` crate.
    - We can also implement `Summary` on `Vec<T>` in our aggregator crate, because the trait `Summary` is local to our `aggregator` crate.
    - But we can't implement external traits on external types. 
    - we cannot implement the `Display` trait on `Vec<T>` within out `aggregator` crate.

### Traits as Parameters
```rust
pub fn notify(item: &impl Summary){
    println!("Breaking News! {}", item.summarize);
}
```
- instead of a concrete type for the `item` parameter, we specify `impl` keyword and the trait name.
- This parameter accepts any type that implements the specified trait.

### Trait bound Syntax
- We have a longer form for `impl Trait` syntax.
```rust
pub fn notify<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}
```
- this longer for is equivalent to the example above but is more verbose.
- We place trait bound with the declaration of the genric type parameter after a colon and inside angle bracket.
- We can have two parameter that implement  `Summary`, with `impl` it would look like
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary){}
```
- however with trait bound it would look like
```rust
pub fn notify<T: Summary>(item1: &T, item2: &t){}
```
- Using trait bound is better,

### Specifying multiple Trait bounds with the + syntax
- We can also speficy more than one trait bound. 
- example:we want `notify` to use display formatting as well as `summarize` on `item`
- We can do so using the `+` syntax
```rust
pub fn notify(item: &(impl Summary + Display)){

}
```
- The `+` syntax is also valid with trait bound on generic type
```rust
pub fn notify<T: Summary + Display>(item: &T){}
```
### Clearer trait bound with `where` clause
- Using too many trait bound has it's downsides. 
- Each generic has it's own trait bound, so functions with multiple generic type parameters can contain  lot of trait bound information between the function's name and it's parameter list, making the function hard to read.
- Rust has alternative syntax for specifying trait bounds inside a `where` clause after the function signature
- instead of writing this
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {

}
```
- we can use a where clause, like this:
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone
    U: Clone + Debug
{}
```
### Returning Types that implement Traits
- We can also use the `impl Trait` syntax in the return position to return a value of some type that implments a trait
```rust
fn returns_summarizable() -> impl Summary{
    Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}
```
- The ability to specify a return type only by the trait it implements is especially useful in context of closures and iterators.
- Closures and iterators create types that only the compiler knows or types that are very long to specify.
- The `impl Trait` syntax lets us concisely specify that a function returns some type that implements the `Iterator` trait without needing to write out a very long type.
- However, we can only use `impl trait` if you're returning a single type.
- Example, this code wont work
```rust
fn returns_summarizable(switch: bool) -> impl Summary{
    if switch {
        NewsAritcle{
            ...
        }
        else{
            Tweet{
                ...
            }
        }
    }
}
```
### Using Trait bounds to Conditionally implement methods
- By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods for conditionally for types that implement the specified traits.
- Example
```rust
use std::fmt::Display;

struct Pair<T>{
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        }
        else {
            println!("The largest memner is y = {}", self.y);
        }
    }
}
```
- The type `Pair<T>` always implements the `new` function to return a new instance of type `Pair<T>`
- But in next `impl` block, `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd` train that enables the comparison and the `Display` trait that enables printing.
- We can also conditionally implement a trait for any type that implements another trait. 
- Implementations of a trait on any trait that satisfies the trait bounds are called `blanket implementations` and are extensively used in the Rust standard library. 
- Example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to 
```rust
impl<T: Display> ToString for T{
    ...
}
```
- Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.
- Trait and trait bounds lets us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particulatr behaviour. 
- The compiler can then use the trait bound information to check that all the concrete types used with out code provide the correct behavior.
- In dynamically typed language, we would get error at runtime if we call a method on a type with didn't define the method.
- But Rust moves these errors to compile time so we're forced to fix the problems before our code is even able to run. 
- Additionally we don't have to write code that checks for behaviour at runtime because we've already checked at compile time.
- Doing so improves performance withour having to give up the flexibility of generics.
