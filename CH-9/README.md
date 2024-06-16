# Error Handling
- Rust groups error into two major categories: recoverable and unrecoverable errros.
- For recoverable error, such as `file not found` error, we must likely want to report the problem to the user and retry the operation.
- Unrecoverable errors are always symptoms of bugs, we want to immediately stop the program
- Most languages don't distinguish between two kinds of errors and handle both in the same way, using mechanisms such as exceptions.
- Rust doesnot have exceptions. Instead it has the type `Result<T, E>` for recoverable error and `panic!` macro that stops execution when the program encounters an unrecoverable error.

## Uncoverable Erros with `panic!`
- Ther are two ways to cause panic in practice
    - by taking an action that causes our code to panic
    - by explicitly calling the `panic!` macro
- By default, these panics will print a failure message, unwind, clean up the stack and quit.
- Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of panic.

### Unwinding the stack or Aborting in response to a Panic
- By default, when a panic occurs, the program starts `unwinding`, which means Rust walks back up the stack and cleans up the date from each function it encounters.
- This walking back and cleaning up is a lot of work.
- Rust therefor allows us to chose the alternative of immediatly `aborting`, which ends the program without cleaning up.
- Memory that the program was using will then need to be cleaned up the by the operating system.
- If we need to make the resulting binary as small as possible, we can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropiate `[profile]` section in out `cargo.toml` file.

```rust
fn main(){
    panic!("Crash and burn");  
}
```

### Using a `panic!` backtrace
```rust
fn main(){
    let v = vec![1, 2, 3];
}
```
- In this case rust will panic.
- Rust will stop execution and refuse to continue
- We can set `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error.
- A backtrace is a list of all the function that have been called to get to this point.

### Recoverable Errors with Result
- The `Result` enum is defined as having two variants `Ok` and `Err`, as follows:
```rust
enum Result<T, E>{
    Ok(T),
    Err(E),
}
```
- The `T` and `E` are generic type parametes.
- `T` represents the type of value that will be returned in a success care within the `Ok` variant
- `E` represents the type of the error that will be returned in a failure case within the `Err` variant.
- Function that returns a `Result` value because the function could fail
```rust
use std::fs::File;
fn main(){
    let greeting_file_result = File::open("hello.txt");
}
```
- The return type of `File::open` is a `Result<T, E>`. 
- The generic parameter `T` has been filled in by the implementation of `File::open` with the type of the success value `std::fs:File`, whihc is a file handle.
- The type of `E` used in teh error value is `std::io::Error`. 
- The return type means the call to `File::open` might suceed and return a file handle that we can read from or write to.
- The function call might also fail, the file might not exist, or we might not have permission to access the file.
- The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information.
- In the case where `File::open` succeeds, the value in the variable `greeting_file_result` will be an instance of `ok` that contains a file handle.
- In the case where it fails, the value in `greeting_file_resutl` will be an instance of `Err` that contains more information about the kind of error that happened.
- Code to take different action depending on the value `File::open`
```rust
use std::fs::File;
fn main(){
    let greeting_file_resutl = File::open("hello.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error);
    };
}
```

### Matching with different Error
- The above code will `panic!` no matter why `File::open` failed. 
- We want to take different action for different reasion
    - if `File::open` failed because thefile doesn't exist, we want to create the file and return the handle to the new file.
    - If `File::open` failed for any other reasion - we still want the code to panic.
```rust
use std::fs::File;
use std::io::ErrorKind;
fn main(){
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```
- The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library.
- This struct has a method `kind` that we wan call to get an `io::ErrorKind` value. 
- The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of error that might result from an `io` opereation.
- The variant we want to use is `ErrorKind::NotFound`, which indicated the file we're trying to open doesn't exist yet.

### Shortcut for Panic on Error:  `unwrap` and `expect`
- The `unwrap` method is a shortcut method implemented just like the `match` expression we wrote. 
- If the `Result` valueis the `Ok` variant, `unwrap` will return the value inside the `Ok`.
- if the `Result` is the `Err` variant, unwrap will call the `panic!` macro for us.
```rust
use std::fs::File;
fn main(){
    let greeting_file = File::open("hello.txt").unwrap();
}
```
- if we run this code without a `hello.txt` we'll see an error message from the `panic!` call that the `unwrap` method makes.
- The `expect` method lets us also choose the `panic!` error message. 
- Using `expect` instead of `unwrap` and providing good error messages can convey our intent and make tracking down the source of a panic easier.
```rust
use std::fs::File;

fn main(){
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in the project");
}
```

### Propogating Errors
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result{
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```
- The return type is `Result<String, io::Error>`. This means that the function returning a value of the type `Result<T, E>` where the generic parameter `T` has been filled in with the concrete type `String` and generic type `E` has been filled with the concrete type `io::Error`
- If this code succeeds without any problem, the caller wil recieve an `Ok` value that holds a `String`
- If this function encounters any problems, the calling code will recieve an `Err` value that holds an instance of `io::Error` that contains more information about what the problems were.
- In the case of `Err` instead of calling `panic!` we use the `return` keyword to return early out of the function and pass the error value from `File::open`
- If `read_to_string` fails, we return the error value in the same way that we returned the error value in the `match` that handled the return value of `File:open`. We don't need to explicitly say `return`, becuase this this is the last expression in the function.
- The code that calls this code will then handle getting either an `Ok` value that contains a username or an `Err` value that contains `io::Error`. 
- it's upto the calling code to decide what to do with those values.
- This Pattern of propogating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.

### A shortcut to propogating error: the `?` operator
```rust
use std::fs::File;
use std::io::{self, Read};

let read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt");
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);
}
```
- the `?` placed after a `Result` value is defined to work in almost the same way as the `match` expresssion we defined to handle the `Result` value earlier.
- if the value of the `Result` is an `Ok` the value inside the `Ok` will get returned from this expression, and the program will continue.
- If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propogated to the calling code.
- Difference between `match` and `?` operator:
    - error value that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert value from one type into another 
    - When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function.
    - This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reason.
- Here in the above code `?` at the end of the `File::open` call will return the value inside an `Ok` to the variable `username_file`.
- If error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code.
- We could shorten this code much further
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username);
}
```
- Here instead of creating a variable `username_file`, we've chained the call to `read_to_string` directly onto the result of `File::open("hello.txt")?`.
- We still have a `?` at the end of the `read_to_string` call and we still return an `Ok` value containing `username` when both `File::open` and `read_to_string` suceed rather tahn returning error.
- We can shorten this code more further
```rust
use std::fs;
use stf::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### Where the `?` operator can be used
- The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on.
- Because the `?` operator is defined to perform an early return of a value out of the function. 
- IF we use `?` operator in main function 
```rust
use std::fs::File;
fn main(){
    let greeting_file = File::open("hello.txt")?;
}
```
- The operator `?` follows the `Result` value returned by `File::open`, but this `main` function has the return type of `()` not `Result` we will get error.
- The error points out that we're only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`
- Two ways of handling the error
    1. Change the return type of your function to be compatible with the value you're using the `?` operator.
    2. Use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way possible.
- `?` operator can also be used with `Option<T>` values, we can use `?` on only on `Option<T>` in a function that returns an `Option`
- The behaviour of `?` operator when called on an `Option<T>` is similar to it's behaviour when called on a `Result<T, E>`
    - if the value is `None`, the `None` will be returned early from the function at that point.
    - if the value is `Some`, the value inside `Some` is the resulting value of the expression and the function continues
- Example
```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```
- This function returns `Option<char>` because it's possible that there is a character there, but it's also possible that there isn't. 
- It takes `text` string slice argument and calls the `lines` method on it, which returns an iterator over the lines in the string.
- We want to examine the first line, we call the `next` on the iterator to get teh first value from the iterator.
- if the `text` is empty string, this call to `next` will return `None` in which case we use `?` to stop and return `None` from the function.
- if `text` is not the empty string, `next` will return a `Some` value containing a string slice of the first line in `text`
- The `?` extracts the string slice, and we can call `chars` on the string slice to get an iterator of it's characters.
- We call `last` to return the last item in the iterator.
- We can use the `?` operator on a `Result` in a function that returns `Result` and we can use `?` operatoe  on a option in a function that returns `Option`, but we cannot mix and match.
- The `main` function can also return a `Result<(), E>`
```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```
- `Box<dtn Error>` mean "any kind of error"
- using `?` on a `Result` value in `main` function with the error type `Box<dyn Error>` is allowed, because it allows any `Err` value to be returned early. 
- Even though the body of this `main` function will only ever return errors of type `std::io::Error`, by specifying `Box<dyn Error>` this signature will continue to be correct even if more code that return other error is added to the body of main.
- When a `main` function returns a `Result<(), E>` the executable will exit with a value of `0` if `main` returns `Ok(())` and will exit with a nonzere value if `main` returns an `Err` value

## To `panic` or not to `panic`
- when code panics there is no way to return.
- when you chose to return a `Result` value, you give calling code options. 
- The calling code could chose to attempt to recover in a way that's appropiate for it's situation, or it could decide that an `Err` value in this case in unrecoverable, so it can call `panic!` and turn your recoverable code into unrecoverable one.
- Returning `Result` is a good default choice when you're defining a function that might fail.

### Examples, Prototype Code and Tests
- When we are writing an example to illustrate some concept, also including robust error-handling code can make the examples less clear.
- `unwrap` and `expect` methods are very handy when prototyping, before you're ready to decide how to handle errors. They leave clear markers in your code for when you're ready to make your program more robust.
- if a method call fails in a test, you'd want the whole test to fail, even if that method isn't the functionality under test. Because `panic!` is how a test is marked as failure, calling `unwrap` or `expect` is exactly what should happen.

### Cases in which you have more information than the compiler
- it would be appropiate to call `unwrap` or `expect` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn't something that compiler understands
- If we can ensure by manually inspecting the code that we'll never have an `Err` variant, it perfectly acceptable to call `unwrap` and better to call `expect` with the reason that this won't fail.
```rust
use std::net::IpAddr;

let home : IpAddr = "127.0.0.1"
                    .parse()
                    .expect("Hardcoded IP Address should be valid")

```