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

## Validating references with Lifetimes
- Rather than ensuring that a type has the behaviour we want, lifetimes ensure that references are valid as long as we need them to be.
- Every reference in Rust has a `lifetime`, which is the scope for which that reference is valid. 
- Most of the time lifetimes are implicit and inferred, just like most of the time, types are inferred.
- We must annotate types when multiple types are possible.
- In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a different ways.
- Rust requires us to annotate the relationship using generic lifetime parameter to ensure the actual regerences used at runtime will definitely be valid.
### Preventing dangling references with Lifetimes
- The main aim of lifetimes is to prevent `dangling references`, which cause a program to reference data other than the data it's intended to reference
- Example
```rust
fn main(){
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
}
```
- the outer scope declares `r` with no intial value, inner scope declares a variable named `x` with initial value of 5
- inside the inner scope we attempt to set the value of `r` as a reference to `x`
- Then the inner scope ends, and we attempt to print the value in `r`.
- This code won't compile because what the value `r` is reffering has gone out of scope before we try to use it.
- The variable `x` doesn't live long enough.
- `r` is still valid for the outer scope, because it's scope is larger, we can it lives longer.
- if rust allows this code to work, `r` would be referencing memory that was deallocated when `x` went out of the scope, and anything we tried to do with `r` wouldn't work correctly.
- How does rust determine that this code is valid? It uses a Borrow checker.
### The Borrow Checker
- The Rust compiler has a `borrow checker` that compares scopes to determine whether all borrows are valid.
- Example code with annotations showing the lifetimes of the variable.
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```
- We've annotated the lifetime of `r` with `'a` and lifetime of `x` with `'b`.
- the inner block `'b` is much smaller than the outer `'a` lifetime block.
- At compile time, rust compares the size of the two lifetimes and  sees that `r` has a lifetime of `'a` but that it refers to memory  with a lifetime of `'b`. 
- The program is rejected because `'b` is shorter that `'a` the subject of the reference doesn't live a long a the reference.
- Fixed code
```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```
- Here `x` has a lifetime of `'b` which in this case is larger than `'a`.
- this means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

### Generic Lifetimes in Functions
- Ecample for a function that returns longer of two string slices. 
- This function will take two string slices and return a single string slice. 
```rust
fn main(){
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {result}");
}
```
- we want the function to take string slices, which are references, rather than strings because we don't want the `longest` function to take ownership of its parameter.
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y 
    }
}
```
- This won't compile, we get the error that talks about lifetimes.
- The return type needs a generic lifetime parameter on it because Rust can't tell whether the reference being returned refers to `x` or `y`. 
- We also don't know either, because the `if` block in the body of this function returns a reference to `x` and the `else` block returns a reference to `y`
- When we are defining this function, we don't know the concrete values that will be passed into this function, so we don't know whether the `if` case or the `else` case will execute. 
- we also don't know the concrete lifetimes of the references that will be passed in, so we can't look at the scopes as we did earlier to determine whether the reference we return will always be valid.
- The borrow checker can't determine this either, because it doen't lnow how the lifetimes of `x` and `y` relate to the lifetimes of the return value.
- To fix this error we'll add generic lifetimes parameter that define the relationship between the references so the borrow checker can perform it's analysis.

### Lifetime Annotation Syntax
- Lifetime annotation don't change how long any of the references live.
- Rather they describe the relationship of the lifetimes of multiple references to each other without affecting the lifetimes.
- Just as functions can accept any type when the signature specifies a generic type parameter, function can accept references with any lifetime with by specifying generic lifetime parameter.
- The names of lifetime paramter must start with an apostrophe `'` and are usually all lowercase and very short.
- Most people use the name `'a` for the first lifetime annotation.
- We place lifetime parameter annotation after the `&` of referencem using a space to separate the annotation from the reference type.
- Examples
```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime.
```
- one lifetime annotation by itself doesn't have much meaning, because annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.

### Lifetime Annotation in Function Signature
- To use lifetime annotation in function signature, we need to declare the generic `lifetime` parameter inside angle brackets between the function name and the parameter list.
- We want the signature to express the following contraint: the returned reference will be valid as long as both the paramters are valid.
- This is the relationship between lifetimes of the parameters and the return value.
- We'll name the lifetime `'a` and then add it to each reference
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- The function signature tells Rust that for some lifetime `'a` the function takes two parameters, both of which are string slices that live atleast as long as lifetime `'a`
- The function signature also tells that the string slice returned from the function will live atleast as long as lifetime `'a`. 
- In practice, it means that the lifetime of the reference returned by `longest` function is the same as the smaller of the lifetime of the values reffered by the function argument.
- When we specify the lifetime parameter in function signature, we're not changing the lifetimes of any values passed in or returned. 
- Rather we're specifying that the borrow checker should reject any values that don't adhere to these constraints.
- When annotating lifetimes in function, the annotations go in the function signature, not in the function body.
- The lifetimes annotations become part of the contract of the function, much like the types in the signature. 
- Having function signature contain the lifetime contract means the analysis the Rust compiler does can be simpler.
- If there's a problem with the way a function is annotated or the way it is called, the compiler errors can point to the part of our code and the contraints more precisely.
- If Rust compiler made more inferences about what we intended the relationships of the lifetimes to be, the compiler might only be able to point to use of our code many steps away from the cause of the problem.
- When we pass concrete references to `longest` the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`.
- In other words, the generic lifetimes `'a` will get the concrete liftime that is equal to the samller of the lifetimes of `x` and `y`. 
- The returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`
- Examples
```rust
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is {result}");
    }
}
```
- `string1` is valid until the end of the outer scope, `string2` is valid till the end of inner scope, the `result` references something that is valid untill the end of the inner scope.
- Borrow checker approves this.
```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
```
- We will get error
- The error shouws that for `result` to be valid for the `println`, `string2` will need to be valid untill the end of the outer scope.
- `string1` is longest and will be in scope till end of outer scope.
- However, compiler can't see that the reference is valid in this case.
- We've told Rust that the lifetime of the reference returned bt thee longest function is the same as the smallter of the lifetimes of the reference passed in. That's why compiler disallows the code.

### Thinking in terms of Lifetimes
- The way ww need to specify lifetime parameters depends on what our function is doing.
- Example, if we changed the implementation of the `longest` function to always return the first parameter rather than the longest string slice, we wouldn't need to specify a lifetime on the `y` parameter.
```rust
fn longest<'a> (x: &'a str, y) -> &'a str {
    x
}
```
- Here lifetime of `y` does not have any relationship with the lifetime of `x` or the return value.
- When returning a reference from a function, the lifetime parameter for the return type needs to match the liftime parameter for one of the paramters.
- if the reference returned does nto refer to one of the parameters, it must refer to a value created in the function.
- This would be a dangling reference because the value will go out of scope at the end of the function.
```rust
fn longest<'a> (x: &'a str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
- This implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameter at all.
- Ultimately, lifetime syntax is about connecting the lifetime of various parameters and return value of function. Once they're connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise voilate memory safety.

### Lifetime Annotations in Struct Definitions
- We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct's definition
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me ishmael, Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find a .");
    let i = ImportantExcerpt{
        part: first_sentence,
    }
}
```
- This annotation means an instance of `ImportantExcerpt` can't outlive the reference it hold in it's `part` field.

### Lifetime elision
- Example code::
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}
```
- This code compliled without lifetime annotation.
- Reason is historical: in early versions of Rust, this code wouldn't have complied because every reference needed an explicit liftime.
```rust
fn first_word<'a> (s: &'a str) -> &'a str{ }
```
- After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotation over and over in particular situation.
- These situations were predictable and followed a few deterministic pattern.
- The programmers programmed these patterns in to the compilers code so the borrow checker could infer the lifetimes in these situations and wouldn't need explicit annotations.
- The patterns programmed into Rust's analysis of references are called the `Lifetime elision` rules.
- These rules aren't rules for programmers to follow; they're set of particular cases that the complier will consider, and if your code fits these cases, we don't need to write the lifetimes explicitly.
- The elision rules don't provide full inference. If there is still ambiguity as to what lifetime references have, the compiler won't guess what the lifetime of the remaining references should be.
- Instead of guessing, the compiler will give us an error that we can resolve by adding lifetime annotations.
- Lifetime on functions or method prameters are called `input lifetimes` and lifetimes on return values are called `output lifetimes`
- The complier uses three rules to figure out the lifetime of the references when there aren't explicit annotations.
- First rule applies to input lifetimes, second and third applies to output lifetime.
- If the compiler gets to the end of the three rueles and there are still references for which it can't figure out lifetimes, the compiler will stop with an error.
- First Rule: the complier assigns lifetime parameter to each parameter that's a reference. 
    - a function with one parameter gets one lifetime parameter `fn foo<'a> (x: &'a i32)`
    - a function with two parametes gets two separate lifetime parameters `fn foo<'a, 'b> (x: &'a i32, y: &'b i32)`
- Second Rule: if there is exactly one input lifetime parameter, the lifetime is assigned to all output lifetime parameters `fn foo<'a> (x: &'a i32) -> &'a i32`
- Third Rule: if ther are multiple input lifetime paramters, but one of them is `&self` or `&mut self` because  this is a method, the lifetime of `self` is assigned to all output lifetime parameter.
- Applying rules on `first_word` functin
    - The complier applies the first rule which specifies that each parameter gets it's own lifetime
    - `fn first_word<'a>(s: &'a str) -> &str`
    - The second rule applies because there is only one input lifetime. The lifetime of one input parameter get's assigned to the output lifetime, so the signature is now
    - `fn first_word<'a>(s: &'a str) -> &'a str`
    - Now all the references in this function signature have lifetimes and compiler can continue its analysis.
- Applying rules on `longest` function
    - `fn longest(x: &str, y: &str) -> &str`
    - applying first rule
    - `fn longest<'a, 'b> (x: &'a str, y: &'b str) -> &str`
    - Second rule doesn't apply since ther more than one input lifetimes
    - Third rule doesn't apply either, because `longest` is a function rather than a method.

### Lifetime Annotations in Method Definitions
- When we implement method on a struct with lifetimes, we use the same syntax as that of generic type parameters. 
- Lifetime names for struct field always need to be declared after the `impl` keyword and then used after the structs name, because those lifetimes are part of the structs type.
- in method signature inside the `impl` block, references might be tied to the lifetimes of references in the structs field or they might be independent.
- In addition, the lifetime elision rules often make it so that the lifetime annotations are not neccessary in method signatures.
- Example
```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3 
    }
}
```
- The lifetime paramter after `impl` and its use after the type name are required, but we're not required to annotate the lifetime of the reference to the `self` becuase of the first elision rule.
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) {
        println!("Attention Please: {announcement}");
        self.part
    }
}
```
- There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self` and `announcement` their own lifetime.
- Then because one of the parameter is `&self` the return type gets the lifetime of `&self`, and ll the lifetimes have been accounted for.

### The Static Lifetime
- `'static` denotes that the affected reference can live for the entire duration of the program.
- All string literals have `'static` lifetime
```rust
let s: &'static str = "I have a static lifetime";
```
- The text of this string is stored directly in the programs binary, which is always avaliable.
- Before specifying `'static` as the lifetime for a reference, think about whether the reference you have actaully lives for the entire lifetime of our program or not.

### Generic Type Parameter, Trait Bounds, and lifetime together
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- This has an extra parameter named `ann` of the generic type `T` which can be filled by any type that implements the `Display` trait as specified by `where` clause.
- Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle bracket after the function name.
