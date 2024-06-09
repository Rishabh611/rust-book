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

## Storing UTF-8 Encoded Text with String

### What is a String?
- Rust has only one String type in the core language, which is the string slice `str` that is usually seen in it's borrowed form `&str`.
- String literals, are stored in the program's binary and are therefore string slices.
- The `String` type, which is provided by Rust's standard library rather than coded into the core langauge is a growable, mutable, owned, UTF-8 encoded String type.

### Creating a new String
- Many of the same operations available with `Vec<T>` are available with `String` as well.
- String is actually implemented as a wrapper around a vectory of bytes with some extra guarantees, restrictions, and capabilities.
```rust
let mut s = String::new();
```
```rust
let data = "initial contents";
let s = date.to_string();

// the method also works on a literal  directly:
let s = "initial contents".to_string();
```
```rust
let s = String::from("initial contents");
```
### Updating a String
- We can use `+` operator or the `format!` mactor to concatenate `String` values.
#### Appending to a String with `push_str` and `push`
```rust
let mut s = String::from("foo");
s.push_str("bar");
```
- the `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter.
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```
- the `push` method takes a single character as a parameter and adds it to the sting
```rust
let mut s = String::from("lo");
s.push('l');
```
#### Concatenation with the + operator or the `format!` macro
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used.
```
- The `+` operator uses the `add` method, whose signature looks like this:
```rust
fn add(self, s:&str) -> String{

}
```
- First `s2` has an `&`, meaning we're adding a reference of the second string to the first string.
- This is because of the `s` parameter in the `add` function: we can only add a `&str` to a `String`; we can't add two `string` values together.
- But type of `&s2`is `&String` in the above example, not `&str`. How can this compile?
- Compiler is able to `coerce` the `&String` argument into `&str`. 
- when we call `add` method, Rust uses a deref coercion which here turns `&s2` into `&s2[..]`
- But `add` does not take ownership of the `s` parameter, `s2` will be a valid string after this operation
- We also see that `add` takes ownership of `self` because `self` does not have an `&`. 
- This means `s1` will be moved into the `add` call and will no longer be valid after taht.
- if we need to concatenate multiple strings, the behaviour of the `+` operator gets unwieldy
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "_" + &s2 + "_" + &s3;
```
- the `s` will be `tic-tac-toe`.
- For more complicated String combining, we can instead use the `format!` macro
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{s1}-{s2}-{s3}");
```
- the code generated by the `format!` macro uses reference so that this call doesn't take ownership of any of its parameters.

### Indexing into Strings
```rust
let s1 = String::from("hello");
let h = s1[0];
```
- If we try to access parts of `String` using a indexing syntax in Rust, we'll get an error.
- Rust Strings don't support indexing. 
- We need to understand how Rust stores strings in memory

#### Internal Representation
- A `String` is a wrapper over a `Vec<u8>`.
```rust
let hello = String::from("Hola");
```
- in this case `len` will be4, which means the vector storing the sring "Hola" is 4 byte long.
- wach of these letters take 1 byte when encoded in UTF-8. 
```rust
let hello = String::from("Здравствуйте");
```
- On first look it looks like len of this string is 12. 
- In fact, Rust's answer is 24.
- That's the number of bytes it takes to encode this string in UTF-8, because each Unicode scalar value in this String takes 2 bytes of storage.
- Therefore, an index into the string's byte will not always correlate to a valid Unicode scalar value.
```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```
- this is an invalid Rust code.
- we know that `answer` will not be `3`, the first letter.
- when encoded in UTF-8, the first byte of `3` is `208` and the second is `151`, so it would seem that `answer` should be in fact `208` but `208` is not a valid character on its own.
- Users generally don't want the byte value returned, even if the string contains only Latin letter: if &"hello"[0] were valid code that returned the byte value, it would return `104` not `h`
#### Bytes and Scalar values and Grapheme Clusters
- In UTF-8 there are actually three relevant ways to look at strings from Rust's perspective
    - as bytes
    - as scalr values
    - grapheme clusters
- “नमस्ते” in Hindi, stores as a vector of `u8` values that look like this
```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```
- 18 bytes, this is how computer ultimately store this data.
- if we look at them as Unicode scalar values, which are what Rust's `char` type, those bytes look like:
```
['न', 'म', 'स', '्', 'त', 'े']
```
- there are six `char` values, but fourth and sixth are not letters: they're diacritics that don't make sense on their own.
- if we look at them as grapheme clusters, we'd get what a person would call the four letters that make up the word.
```
["न", "म", "स्", "ते"]
```
- Rust provides different ways of interpreting the raw sting data that computers store so that each program can chose the interpretation it needs, no matter what human language the data is in.
- Final reason why rust doesn't allow us to index into `String` to get character is that indexing operations are expected to always take constant time (O(1)).
- it's impossible to guarantee that performace with `String` because rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

### Slicing Strings
- Rather than indexing using [] with a single number, we can use [] with a range to create a sting slice containing particular bytes
```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```
- here `s` will be a `&str` that contains the first 4 bytes of the string. Which means s will be `Зд`
- if we were to try to slice only part of character's byte with something like `&hello[0..1]`, rust would panic at runtime in the same way as if invalid index were accesses in a vector.

#### Methods for iterating Over Strings
- The best way to operate on pieces of strings is to be explicit about whether we want characters or bytes.
- For individual Unicode scalar values, use the `chars` method.
```rust
for c in "Зд".chars(){
    println!("{c}");
}
```
Alternatively, the `bytes` method returns each raw byte
```rust
for b in "Зд".bytes(){
    println!("{b}");
}
```

## Storing Keys with Associated value in HashMaps
### Creating a new HashMap
- We can create hashmap using `new` and add new elements with  `insert`
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
- HashMap is less frequently used so it's not included in the prelude.
- HashMaps have less support from the standard library; there is not built in macro to construct them.
- HashMaps store their data on the heap

### Accessing Values in a HashMap
- We can get value out of HashMap by providing it's key to the `get` method.
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```
- score will have value that's associated with the blue team, and the result will be `10`.
- The `get` method returns an `Option<&V>`
- if there is no value in `scores` it will return `None`
- The program handles the `Option` by calling `copied` to get an `Option<i32>` rather than a `Option<&i32>`, then `unwrap_or`  to set `score` to zero if scores doesn't have an entry for the key.
- we can iterate over a hash map (key/value) pair using a `for` loop
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores{
    println!("{key}: {scores}");
}
```

### HashMaps and ownership
- For types that implement `Copy` trait, like `i32`, the values are copied to the HashMap.
- For owned values like `String`, the values will be moved and the HashMap will be the owner of the values.
```rust
use std::collections::HashMap;

let field_name = String::from("Favourite Color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// field_name and field_value are invalid at this point.
```

### Updating the HashMap
#### Overwriting the values
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 50);

println!("{:?}", scores);
```
- This code will print `Blue: 50`, original value will be overwritten.

#### Adding a value only if the key doesnot exist
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}" scores);
```
- HashMaps have a special API `entry` that takes the key you want to check as a parameter/
- The return values of `entry` method is an `Enum` called `Entry` that represents a value that might or might not exist.
- The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if the key exists, and if not, inserts the parameter as the new value for this key and return a mutable reference to the new value. 
- The above code will print `{Yellow: 50, Blue: 50}

#### Updating value based on old values
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```
- This code will print `{hello:1, world:2, wonderful:1}`
- The `split_whitespaces` returns a iterator over sub-slices, separated by whitespace of the values of text.
- The `or_insert` function return a mutable reference (`&mut v`) to the value for the specified key. 
- here we store that mutable reference in the `count` variable, so in order to assign that value we must need to dereference count using `*`

### Hashing Functions
- By default, `HashMap` uses a hashing function called `SipHash` that can provide resistance to Denial of Service(DoS) attacks involving Hash Tables. 
- This is not the fastest hashing algorithm but trade of for better security that comes with the drop in preformance is worht it.
- if you find that the default hashing function is too slow for your purposes, we can switch  to another function by specifying a different hasher. 
- A `hasher` is type that implements the `BuiltHasher` trait.
