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
