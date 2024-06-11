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

### Shortcust for Panic on Error:  `unwrap` and `expect`
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