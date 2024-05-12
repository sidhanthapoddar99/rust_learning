
- [Rust Cheat Sheet](#rust-cheat-sheet)
- [Variables \& Mutability](#variables--mutability)
  - [Constants](#constants)
  - [Shadowing](#shadowing)
- [Data Types](#data-types)
  - [Scalar Types](#scalar-types)
    - [Integer Types](#integer-types)
    - [Floating-Point Types](#floating-point-types)
  - [Boolean Types](#boolean-types)
  - [Character Type](#character-type)
  - [Compound Types](#compound-types)
    - [Tuples](#tuples)
    - [Arrays](#arrays)
    - [Other Compound Types](#other-compound-types)
- [Functions](#functions)
  - [Returning Values](#returning-values)
- [Control Flow](#control-flow)
  - [`if` Expressions](#if-expressions)
  - [Loops](#loops)
- [Ownership](#ownership)
  - [The Stack and the Heap](#the-stack-and-the-heap)
  - [Ownership Rules](#ownership-rules)
  - [Variable Scope](#variable-scope)
  - [The `string` Type](#the-string-type)
  - [Memory \& Allocation](#memory--allocation)
    - [Data Interaction: Move](#data-interaction-move)
    - [Data Interaction: Clone](#data-interaction-clone)
    - [Data Interaction: Copy](#data-interaction-copy)
  - [Ownership \& Functions](#ownership--functions)
  - [Return Values \& Scope](#return-values--scope)
- [References \& Borrowing](#references--borrowing)
  - [Mutable References](#mutable-references)
  - [Dangling References](#dangling-references)
- [The Slice Type](#the-slice-type)
  - [String Slice](#string-slice)
  - [Other Slices](#other-slices)
- [Structure Related Data](#structure-related-data)
  - [Building a `struct`](#building-a-struct)
  - [`struct` Update Syntax](#struct-update-syntax)
  - [Tuple Structs](#tuple-structs)
  - [Unit-like Structs](#unit-like-structs)
  - [Methods](#methods)
    - [Defining Methods](#defining-methods)
    - [Associated Functions](#associated-functions)
- [Enumerations \& Pattern Matching](#enumerations--pattern-matching)
  - [Methods](#methods-1)
  - [The Option Enum](#the-option-enum)
  - [The `match` Control Flow Operator](#the-match-control-flow-operator)
  - [Patterns That Bind to Values](#patterns-that-bind-to-values)
  - [Matching with `Option<T>`](#matching-with-optiont)
  - [The `_` Placeholder](#the-_-placeholder)
  - [Concise Control Flow (`if` Statements)](#concise-control-flow-if-statements)
- [Packages, Crates \& Modules](#packages-crates--modules)
  - [Packages \& Crates](#packages--crates)
  - [Defining Modules](#defining-modules)
    - [Module Example - Restaurant Library](#module-example---restaurant-library)
    - [Referring to a Module](#referring-to-a-module)
  - [Making Structs \& Enums Public](#making-structs--enums-public)
  - [Bringing Paths into Scope](#bringing-paths-into-scope)
  - [Using External Packages](#using-external-packages)
    - [Nested Paths](#nested-paths)
    - [The `glob` Operator](#the-glob-operator)
  - [Separating Modules into Different Files](#separating-modules-into-different-files)
- [Common Collections](#common-collections)
  - [Vectors](#vectors)
    - [Creating a Vector](#creating-a-vector)
    - [Modifying a Vector](#modifying-a-vector)
    - [Reading Elements of a Vector](#reading-elements-of-a-vector)
    - [Iterating Over a Vector](#iterating-over-a-vector)
    - [Storing Multiple Types with an Enum](#storing-multiple-types-with-an-enum)
  - [Strings - In Depth](#strings---in-depth)
    - [Creating a String](#creating-a-string)
    - [Updating a String](#updating-a-string)
    - [Indexing into Strings](#indexing-into-strings)
    - [Internal Representation](#internal-representation)
    - [Bytes, Scalar Values \& Grapheme Clusters](#bytes-scalar-values--grapheme-clusters)
    - [Slicing Strings](#slicing-strings)
    - [Iterating Over Strings](#iterating-over-strings)
  - [Hash Maps](#hash-maps)
    - [Creating a `HashMap`](#creating-a-hashmap)
    - [Hash Map Ownership](#hash-map-ownership)
    - [Accessing Values](#accessing-values)
    - [Updating a Hash Map](#updating-a-hash-map)
- [Error Handling](#error-handling)
  - [Unrecoverable Errors with `panic!`](#unrecoverable-errors-with-panic)
    - [Using a `panic!` Backtrace](#using-a-panic-backtrace)
  - [Recoverable Errors with `Result`](#recoverable-errors-with-result)
    - [Matching Different Errors](#matching-different-errors)
    - [Shortcuts for Panic on Error](#shortcuts-for-panic-on-error)
    - [Propagating Errors](#propagating-errors)
    - [When to `panic!`](#when-to-panic)


# Rust Cheat Sheet

# Variables & Mutability

Variables are immutable by default. This makes Rust safer and makes concurrency easier.\
**Immutable** means once a value is bound to that variable, it cannot be changed.\
For example:
```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
produces the following output
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
 string is a common operation so Rust has a method for it in the standard library.

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

The `?` operator can only be used on methods which return a `Result` as it is defined using a `match` on the `Result` enum and in methods that return a `Result`. Therefore, we cannot use `?` in `main` as it returns `()`. revious error

Variables can be made **mutable** by adding the `mut` keyword in front of the variable name.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

## Constants

Constants can be declared using the `const` keyword instead of `let`. For constants, the type must be explicitly stated.\
*Note: `mut` cannot be used with constants*\
```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

## Shadowing

A new variable can be declared with the same name as the original. We say the original variable is **shadowed** by the new variable. `let` must be used when shadowing.
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```
*Note: when the inner scope finishes `x` will return to 6.*
Shadowing is different to making a variable mutable. If `let` is removed a compile-time error will appear. Shadowing allows us to do some transformations on a variable then have it return to immutable after. Therefore shadowing effectively makes a new variable

# Data Types

Rust is statically typed, this means it must know the types if all variables at compile time. The compiler can usually infer the type, but sometimes type annotation must be added.
```rust
let guess: u32 = "42".parse().expect("Not a number");
```
This defines `guess` as a `u32`. We have to do this as the `parse()` method could be many different types of number.

## Scalar Types

A scalar type represents a single value. Rust has four scalar types: integers, floating-point numbers, Booleans and characters.

### Integer Types

Signed integers are in the format `ixxx` and unsigned are in the format `uxxx`, where `xxx` is the number of bits. The number of bits allowed in Rust are:
- 8
- 16
- 32
- 64
- 128

An n-bit signed integer, numbers from -(2<sup>n-1</sup>) to (2<sup>n-1</sup>-1) inclusive. For example, `i8` can store from -128 to to 127 (-2<sup>7</sup> to 2<sup>7</sup>-1). Unsigned integers can store from 0 to 2<sup>n</sup>-1, so a `u8` can store from 0 to 255 (2<sup>8</sup>-1)

*Note: signed numbers are stored using two's compliment*

There also exist integer types which scale to the word size of the system
- `isize`
- `usize`

So on a 64-bit system, this would be 64-bits

Integer literals can be written in different bases.
- Decimal -> `98_222`
- Hex -> `0xff`
- Octal -> `0o77`
- Binary -> `0b1111_0000`
- Byte (`u8` only) -> `b'A'`

*Note: `_` is a visual separator to make the number easier to read*

Number literals that can be multiple types can have a type allocation suffix, for example `57u8` defines the value 57 as an unsigned 8-bit integer.

### Floating-Point Types

Rust has two primitive floating-point numbers, `f32` and `f64`. The default type is `f64` as on modern CPUs it is the same speed as `f32` but with higher precision.

## Boolean Types

As expected. Declared with `bool`

## Character Type

Characters are declared with the `char` keyword and single quotes.
```rust

fn main(){
  let c = 'z';
}
```

The character type is 4 bytes and represents a Unicode Scalar Value.

## Compound Types

Compound types can group multiple values into one type.

### Tuples

A tuple is a general list of different types. They have a fixed size.
```rust
fn main(){
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
*Note: the type annotation is optional*
The identifier `tup` binds to the whole object. Pattern matching can be used to destruct a tuple; this is called **destructuring**. The individual elements can be retrieved by passing the index.
```rust
fn main(){
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup; // x = 500, y = 6.4, z = 1
  let a = tup.0;
  let b = tup.1;
  let c = tup.2;
}
```

A tuple without any values, `let x = ();`, is a special type that has only one value, written `()`. This type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don't return any other value.

### Arrays

Every element in an array must have the same type, and the array has a fixed length.

Values in an array are written as comma-separated inside square brackets.
```rust
fn main(){
  let a = [1, 2, 3, 4, 5];
}
```
Arrays are allocated on the stack.

Array type annotation is defined using a type (or value) and the length.
```rust
fn main(){
  let a: [i32, 5] = [1, 2, 3, 4, 5]; // An array of length 5 of type i32
  let b = [3; 5]; // = [3, 3, 3, 3, 3];
}
```

Array elements can be accessed using square brackets.
```rust
fn main(){
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let last = a[4];
}
```

Index out of bounds panic occurs at runtime. The given index is checked to see if it is less than the array length.

### Other Compound Types

Other compound types include [`structs`](#structure-related-data) and [`enums`](#enumerations--pattern-matching).

# Functions

The `main` function is the entry point for the program. The `fn` keyword is used to declare functions.
Function parameters can be passed when defining the function as such:
 ```rust
fn main(){
  another_function(5);
}

fn another_function(x: i32){
  println!("The value of x is: {}", x);
}
 ```

In function declarations parameter types **must** be defined.

Rust is an expression-based language. Statements only return the unit value, whereas expression return the result of the expression. An example statement is `let x = 10;`. An expression could be `5+6`; this returns the value 11.

Calling a function or macro is an expression. Expressions do not include an ending semicolon,
otherwise they will be treated as statements. An example of this is as following with the line `x+1`.
```rust
fn main(){
  let y = {
    let x = 3;
    x+1
  };

  println!("The value of y is {}", y);
}
```

## Returning Values

Returned values are not named, but their type is defined using an arrow.
```rust
fn five() -> i32{
  5
}
```
This value will return the number 5.

# Control Flow

## `if` Expressions

```rust
fn main(){
  let number = 3;
  if number < 5{
    println!("number < 5");
  } else if number == 5{
    println!("number = 5");
  }else {
    println!("number > 5");
  }
}
```

If expressions can be used to assign variables a value.
```rust
fn main(){
  let condition = true;
  let number = if condition {5} else {6};
}
```

## Loops

Rust has three different types of loops: `loop`, `while` and `for`.
`loop` repeats endlessly until it is explicitly told to stop with the `break` keyword.
`while` repeats until a condition evaluates until false.
`for` repeats a certain number of times.

The `continue` keyword will skip the rest of the loop and go to the next iteration. Loop labels can be used with `break` and `continue` to operate on a specific loop.
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

Values can be returned from loops. This can be useful to check if a thread has finished.
```rust
fn main(){
  let mut counter = 0;

  let result = loop{
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
}
```
The while loop uses a condition to check if the loop should continue.
```rust
fn main(){
  let mut counter = 3;
  while number != 0{
    println!("{}!", number);
    number -= 1;
  }
  println!("LIFTOFF");
}
```
A for loop can iterate through a collection or run a specific amount of times.
```rust
fn main(){
  let a = [10, 20, 30, 40, 50];
  for element in a{
    println!("The value is {}", element);
  }
  for number in (1..4).rev(){
    println!("{}", number);
  }
}
```

# Ownership
 Rust has no garbage collection but works on a basis of ownership.

## The Stack and the Heap
The stack is LIFO. Data is `push`ed to the stack and `pop` removes it from the stack.

All data on the stack must have a known, fixed size. Data with an unknown size, or a size that might change must be put on the heap. When data is allocated on the heap, the memory allocator finds a block of data the size that has been requested and returns a pointer to this a pointer to this memory location.

Pushing to the stack is much faster than allocating to the heap.

Accessing data in the heap is much slower than accessing data on the stack.

When calling a function, values passed into the function and the function's local variables get pushed onto the stack. When the function is over, those data get popped off the stack. This is of course an oversimplification as this is optimised using registers and other methods within the compiler; but this is out of the scope of this sheet. 

## Ownership Rules

- Each value in Rust has a variable called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Variable Scope

A scope is the range within a program of which an item is valid.
```rust
#![allow(unused)]
fn main(){
  let s = "hello";
  // s is valid here until }
}
```

The value `s` refers to a string literal. The variable is valid from where it is declared to the end of the current scope.

## The `string` Type

To create a mutable string requires the following declaration.
```rust
fn main(){
  let mut s = String::from("hello");
  s.push_str(", world!"); // push_str() appends to the string
  println("{}", s);
}
```
Each string contains 3 pieces of data: a pointer to the character array, the length and the capacity.

For `let s1 = String::from("hello");`, the data would be:
| Name | Value |
|---|---|
| ptr | 0x... |
| len | 5 |
| capacity | 5 |

This data is stored on the stack.

The character array at `0x...` would appear as:
| Index | Value |
|---|---|
| 0 | h |
| 1 | e |
| 2 | l |
| 3 | l |
| 4 | o |

This is stored on the heap.

## Memory & Allocation

As the size of string literals is known at compile time the memory can be allocated and the data can be hard coded into the executable.

As the size of a mutable string is unknown at compile time, memory must be allocated onto the heap. Therefore
- The memory must be requested from the OS at runtime
- The memory must be returned to the OS when the program is finished using it

The first part is done automatically when the string is created. This is common across many programming languages.

The second is more difficult. In languages with a garbage collector this is done automatically. However Rust has no garbage collector. In languages without a garbage collector this is done manually and for every allocation a `free` is required to prevent excess memory use or premature memory de-allocation. Rust automatically de-allocates memory when it goes out of scope. In Rust, the command for memory de-allocation is `drop` and this is often done automatically at the end of a scope.
 
### Data Interaction: Move
Multiple variables can interact with the same data.

For primitive data types, if a variables is assigned to another primitive then the value is simply copied to the second one. This is because of the `Copy` trait.
```rust 
let x = 5;
let y = x;
```
However, for non-primitive types (for example, strings), this is not the case. 
```rust
let s1 = String::from("hello");
let s2 = s1;
```
When we assign `s1` to `s2` the string data on the stack (`ptr`, `len`, `capacity`) are copied to `s2` but the data on the heap is not copied. This makes it more efficient as the whole character array is not recreated. 

This can cause a problem if both strings go out of scope at once. Both `s1` and `s2` will try to free the same memory on the heap (using `drop`); this is known as a **double free** error. This is a memory safety bug and can lead to memory corruption and/or security vulnerabilities.

Rust avoids this issue by classing `s1` to be no longer valid; therefore it doesn't need to be freed once it is out of scope. 

For example, if you try to use `s1` after `s2` has be initialised, it will thrown an error.
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
```rustc
error[E0382]: use of moved value: `s1`
--> src/main.rs:5:28
  | 
3 | let s2 = s1;
  |     -- value moved here
4 | 
5 | println!("{}, world!", s1);
  |                        ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait 
```

This is effectively a shallow copy, but because `s1` is invalidated, it is called a **move**

### Data Interaction: Clone
If we do want to create a deep copy of an object, we can use the clone method. 
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This means the heap is being copied and so is less efficient an more expensive.

### Data Interaction: Copy
The `Copy` trait can be added to types which are stored entirely on the stack (i.e. fixed size). `Copy` cannot be implemented if the type or any of its types implement `Drop`.

## Ownership & Functions
The semantics of passing a value to a function are similar to those of assigning a value to a variable. Passing a variable to a function will move or copy it, the same as assignment.
```rust
fn main(){
  let s = String::from("hello");  // s comes into scope 

  take_ownership(s);  // s moves into the function and is no longer valid in this scope

  let x = 5;  // x comes into scope

  makes_copy(x);  // x will be copied into the function as it is an i32 and so it 
                  // can still be used in this scope
} // x goes out of scope

fn take_ownership(some_string: String){ // some_string comes into scope
  println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. This frees the memory of some_string
fn makes_copy(some_int: i32){ // some_int comes into scope
  println!("{}", some_int);
} // some_int goes out of scope
```

## Return Values & Scope
Returning values can also transfer ownership.
```rust 
fn main(){
  let s1 = gives_ownership(); // The return value is moved into `s1`

  let s2 = String::from("hello"); // `s2` comes into scope

  let s3 = takes_and_gives_back_ownership(s2);  // `s2` is moved into the method and then the return value is moved into `s3`
}

fn gives_ownership() -> String { // Moves its return value into the calling method
  let s  = String::from("hello");
  s
}

fn takes_and_gives_back_ownership(s: String) -> String { // `s` comes into scope
  s // `s` is returned to the calling function
}
```

Taking ownership of a variable follows the same pattern every time. 

Taking and returning ownership of a variable with every function can be quite tedious so references can be used to prevent this.

# References & Borrowing 

References can be used to access variables without moving them. This allows them to be accessed in different scopes.

```rust
fn main(){
  let s1 = String::from("hello");

  let len = calc_length(&s1);

  println!("The length of {} is {}", s1, len);
}

fn calc_len(s: &String) -> usize{
  s.len()
} // `s` goes out of scope but as `calc_len` doesn't own it, `s` isn't dropped
```

The ampersands denote a reference to the value; allowing for access without moving it. `s` is simply a pointer to the parameter passed, in this case `s` points to `s1`.

To reverse this we can dereference a pointer by using the `*`.

Having references as parameters is called **borrowing**; we do not need to return the value at the end of the function to use it.

## Mutable References

If we try to edit a borrowed reference it will throw an error as the references are immutable. To create a mutable reference we need to use `&mut` instead of a single ampersand.

```rust
fn main(){
  let mut s1 = String::from("hello");

  change(&mut s1);

  println!("{}", s1);
}

fn change(s: &mut String){
  s.push_str(", world!");
}
```

There is a limit with mutable references. Only one mutable reference to a variable can exist within a single scope at one time. This prevents data races.

This issue also arises if immutable references are combined with mutable references in a single scope.

## Dangling References

Unlike other languages, the Rust compiler will prevent dangling references. 

# The Slice Type

A `slice` is another data type without ownership. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 

## String Slice

A string slice is simply a reference to a substring.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

*Note: the lower bound is inclusive and the upper bound is exclusive*

This is stored internally as the starting index and then the length of the slice. 

String literals are stored as slices. They are immutable because the pointer to the beginning of the slice is immutable. 

## Other Slices

Other slices can be created from other contiguous collections (for example vectors and arrays) using the same syntax.

# Structure Related Data

A `struct` is like a tuple but each element has a name and the order is not important. 

```rust
struct User {
  username: String,
  password: String, 
  age: u16,
  loggedIn: bool,
}
```

After the struct has been defined, new instances of it can be created with concrete values.

```rust
let user1 = User {
  loggedIn: false,
  username: String::from("name"),
  password: getHash(&username),
  age: 10,
}
```

Elements of a struct can be accessed and changed similar to other languages. Values can only be changed if the instance is declared as mutable.

```rust
fn main(){
  let mut user = User {
  loggedIn: false,
  username: String::from("name"),
  password: getHash(&username),
  age: 10,
  }

  user.age = 11;
  println!("Username: {}", user.username);
}
```

## Building a `struct`

A method can be written to build a struct and return an instance of it.

```rust
fn make_user(email: String, age: u16, password: String) -> User {
  User {
    email,
    password,
    loggedIn: true,
    age,
  }
}
```

*Note: this uses **field init shorthand** syntax which allows us to not repeat element names if they are exactly the same as the parameter names. For example, `email: email,` becomes `email,`*

## `struct` Update Syntax 

This is useful when you want to make a new struct based on an old one but with some values changed. 

```rust
fn main(){
  let user1 = User {
  loggedIn: false,
  username: String::from("user`"),
  password: getHash(&username),
  age: 10,
  }

  let user2 = User {
    username: String::from("user2"),
    password: getHash(&username);
    ..user1
  }
}
```

This syntax will auto-fill any fields that have not been redefined with their values from the older struct. 

## Tuple Structs

Tuple structs look like normal tuples but benefit from the name of the struct. 

```rust
struct Colour(i32, i32, i32);
struct Point(i64, i64, i64, i64);

let black = Colour(0, 0, 0);
let origin = Point(0, 0, 0);
```

`Point` and `Colour` are still different structs even though they have the same fields. Other than this, these structs behave like tuples. 

## Unit-like Structs

Unit-like structs have no fields (hence the name; they are similar to the unit type - `()`). These can be useful if you want to implement a trait but have no data for it.

## Methods

Methods are simply functions defined inside of a `struct`. Their first parameter is always `self` which references the instance of the struct itself upon which the method will act. 

### Defining Methods

To add a method to a struct we first need an `impl` (implement) block for the struct. Inside this block define a method the same as a function but ensuring the first parameter   `&self`.

```rust
impl User {
  fn logIn(&mut self){
    self.loggedIn = true;
  }
}
```
The method can then be run using typical dot notation. 

```rust
user1.logIn() // The use will then be logged in
```

*Note: unlike C++, Rust has automatic referencing and dereferencing when calling methods. Therefore there is no requirement for pointer and instance operators (`user1.logIn()` vs `user1_ptr->logIn()`).*

### Associated Functions

Associated functions are defined inside the `impl` block but do not take `self` as a parameter. They are often used for constructors which will return a new instance of a struct. 

```rust
struct Rectangle {
  i32: height,
  i32: width,
}

impl Rectangle {
  fn area(&self) -> i64 {
    self.height * self.width
  }

  fn square(dimension: i32) -> Rectangle {
    Rectangle {
      height: dimension,
      width: dimension,
    }
  }
}

let myRectangle = Rectangle {
  height: 10,
  width: 1000,
}

println!("The area of the rectangle is {}", myRectangle.area());

let mySquare = Rectangle::square(917);

println!("The area of the square is {}", mySquare.area());
```

Associated functions are called using a double colon (`::`).

# Enumerations & Pattern Matching

An enumeration (A.K.A. `enum`) allows you to create a type by defining all of its possible values. An instance of an `enum` can only take one value at a time.

For example, IP addresses (for now) can only be of type v4 or v6, so the following enumeration would be appropriate.

```rust
enum IpAddressVersion {
  V4,
  V6,
}
```

`IpAddressVersion` is now a custom type we can use throughout the scope.

Instances of the `enum` can then be defined and used. Double colon notations is used to select the value.

```rust
fn route(IpAddressVersion) {
  ...
}

enum IpAddressVersion {
  V4,
  V6,
}

let version4 = IpAddressVersion::V4;
let version6 = IpAddressVersion::V6;

route(version4);
route(version6);
```

Data can also be inserted directly into the `enum`. This attaches the data to the `enum` value.

```rust
enum IpAddress {
  V4(u8, u8, u8, u8),
  V6(String),
}

let v4 = IpAddress::V4(127, 0, 0, 1);
let v6 = IpAddress::V6(String::from("::1"));
```

This can be beneficial over a `struct` because each value can have a different type of data attached to it.

## Methods 

Methods can also be defined on an `enum` using an `impl` block. These methods also take `self` or references to `self` as the first parameter (unless it's an associated function) and can be called using the dot notation (`enumExample.method()`).

## The Option Enum

This is another `enum` defined in the standard library. It is used very often as it encodes the common scenario of a value being something or nothing. This means the compiler can check that all possible values have been handled. 

Due to Rust's lack of a `null` value, the `Option<T>` enum can be used to create the same effect of a value being absent or not. 

The `Option` enum is defined in the standard library as such.

```rust
enum Option<T> {
  Some(T),
  None,
}
```

*Note: the `Option` enum is included in the prelude.*

Therefore any `Option<T>` enum must be converted to a `T` when being used in situations where `T` is required. This helps to catch the case where the value of `T` is assumed to be non-`null` when it is `null`.

## The `match` Control Flow Operator 

The `match` operator compares a value against a series of patterns and then runs code respective to which pattern matches. 

Patterns can be made up of literal values, variable names, wildcards, etc. 

```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn coinToVal(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}
```

Each part inside the `match` block is called an **arm**. Each arm is divided into a pattern, `=>`, and then the code to run.

If the value matches the pattern, then the respective code is run. If not, the next arm is checked. 

Matches must be exhaustive over the whole `enum` for the code to compile. 

## Patterns That Bind to Values

Match arms can also bind to the parts of the values that match the pattern. 

```rust
#[derive(Debug)]
enum UsState {
  Alabama, 
  Alaska,
  ...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}
```

We can then use a `match` to retrieve the `UsState` value of any quarter.

```rust
fn coinToVal(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
        println!("Quarter from {:?}", state);
        25
      },
  }
}
```

## Matching with `Option<T>`

This can be very useful to safely run code on a value that has the possibility of being `None`.

```rust
fn addOne(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(x) => Some(x+1),
  }
}

let five = Some(5);
let six = addOne(five);
let none = addOne(None);
```

## The `_` Placeholder

Rust has the pattern `_` which allows us to match to anything. This can be useful if you only care about a small range of the possible values of the data type. 

For example, if the data is a `u8` but we only care about the values 1 to 5, we can use `_` in the match.

```rust
let someVal = 0u8;
match someVal {
  1 => 1,
  2 => 2,
  3 => 3,
  4 => 4,
  5 => 5,
  _ => (),
}
```

## Concise Control Flow (`if` Statements)

This is useful if a `match` would ignore all but one of the values. 

For example, if we only cared about the value `3`, we could replace the `match` with an `if let`.

```rust
let someVal = Some(3u8);

match someVal {
  3 => println!("three"),
  _ => println!("Not a three"),
}

// Can be replaced with

if let Some(3) = someVal {
  println!("three");
} else {
  println!("Not a three!");
}
```

An `else` clause can also be attached to run code for any pattern that doesn't match the `if let` clause.

# Packages, Crates & Modules

Rust and Cargo have many features to help manage larger projects. 

**Packages:** A Cargo feature that lets you build, test and share crates

**Crates:** A tree of modules that produce a library or an executable

**Modules:** Let you control the organisation, privacy and scope of paths

**Paths:** A way of naming items

## Packages & Crates

The **crate root** is a source file that the Rust compiler starts from and makes up the root module of the project. 

A package is one or more crates that provides functionality. A package contains a `Cargo.toml` file which describes how to build the package. A package must contain zero or one library crate, and as many binary crates as desired. But it must contain at least one crate. 

*Note: all the functionality of a crate is defined within the crate's namespace*

## Defining Modules

Modules allow us to organise the code within crates for ease of reuse and for better readability. Modules also define the privacy of items (whether an item can be used by outside code (`public`) or not (`private`)).

### Module Example - Restaurant Library

We will write a library crate to help model a restaurant. It will have both front and back of house methods. 

First we must run `cargo new --lib restaurant`, then put the following code into `src/lib.rs`.

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist(){}
    fn seat_at_table(){}
  }

  mod serving {
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
  }
}
```

Modules are defined using the `mod` keyword followed by the name of the module.

### Referring to a Module

Rust supports both absolute and relative paths to modules inside of a crate. Absolute paths start from the crate root by using a crate name or a literal crate. A relative path starts from the current module and uses `self`, `super` or an identifier inside the current module. Relative paths start with `super` to start the path inside of the parent module (akin to `../` in filesystem paths).

Both types of paths are followed by one or more identifiers separated by double colons (`::`).

We can now expand the restaurant example from above. 

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist(){}
    fn seat_at_table(){}
  }

  mod serving {
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
  }
}

pub fn eat_at_restaurant(){
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::seat_at_table();
}
```

The function `eat_at_restaurant()` is now defined within the crate's root. It uses the `pub` keyword to expose it to the crate's public API.

However, this code will fail to compile due to Rust's privacy boundaries. In Rust, everything  is private by default. A method cannot use a child item's private item, but can use an ancestor's private items. 

For our example to compile, we need to add `pub` to both `hosting` and the methods inside of it. We do not need to add `pub` to `front_of_house` because it is defined inside the same module as `eat_at_restaurant()`.

## Making Structs & Enums Public

The process of making structs and enums public is similar to that of methods but with some extra details. 

If we use `pub` before the struct definition, the struct will be public but the fields will still be private. We can then denote which fields inside the struct should be public. If one or more of the fields are still private, then an associated function constructor must exist otherwise no instances of the struct could ever be created. 

If you add `pub` before the definition of an enum, all values inside the enum will be public. 

## Bringing Paths into Scope

We can bring a path into scope with the `use` keyword and then use items inside of it as if they're local items. This means we don't have to write the whole path every time we want to use an item. 

This can help simplify our restaurant example. 

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist(){}
    pub fn seat_at_table(){}
  }

  mod serving {
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
  }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
  hosting::add_to_waitlist();
  hosting::seat_at_table();
}
```

We can also provide these modules new names using the `as` keyword.

```rust
use std::io::Result as IoResult;
```

These names can be re-exported to allow external code to call items brought into scope. To do this, we can use `pub use`.

## Using External Packages

To use an external package, first it must be declared inside the `Cargo.toml` file. 

For example, if `rand` was required, `Cargo.toml` would need to include the following. 

```
[dependencies]
rand = "0.5.5"
```

This tells Cargo to download the `rand` package (version 0.5.5) and make it available in the project. 

Then to bring it into scope, we need to include a `use` line starting with the name of the dependency. 

### Nested Paths

If multiple items from the same package are required, we can use a nested path to save space. 

```rust
use std::io;
use std::cmp::Ordering;

// Can be replaced with

use std::{io cmp::Ordering};
```

`self` can also be used in nested paths.

```rust
use std::io;
use std::io::Write;

// Can be replaced with

use std::io::{self, Write};
```

### The `glob` Operator

We can bring all public items defined in a path into scope using the `glob` operator, `*`.

```rust
use std::collections::*;
```

## Separating Modules into Different Files

When modules get large, they can be separated into different files. This makes the code easier to navigate. 

For example, if the `front_of_house` module was defined in `src/front_of_house.rs`, the root file (`src/lib.rc` or `src/main.rs`) would have to include the following lines. 

```rust
mod front_of_house;
```

Using a semi-colon instead of brackets after the `mod` name tells Rust to load the contents from another file with the same name as the module. 

This can also be used if the nested modules are moved into directories. If we moved the `hosting` module into the directory `src/front_of_house/`, we can still use `pub use crate::front_of_house::hosting;`.

# Common Collections

Collections can contain multiple values; unlike the built-in array and tuple types, these are stored on the heap. This means the amount of data does not need to be known at compile time and can resize during runtime. 

A **vector** allows you to store a variable number of values next to each other.

A **string** is a collection of characters. 

A **hash map** allows you to associate a value with a particular key.

There are other collections included in the standard library which can be found [here](https://doc.rust-lang.org/stable/std/collections/).

## Vectors

Vectors allow you to store more than one value in a single contiguous data structure. Vectors can only store values of one type and are useful for lists of items. 

### Creating a Vector

A new, empty vector can be created using `Vec::new()`. When creating a vector, type annotation is required. 

```rust
let v: Vec<i32> = Vec::new();
```

If the vector is being created with initial values, the type can be inferred and type annotation becomes unnecessary. The `vec` macro can be useful for this. 

```rust
let v = vec![1, 2, 3];
```

### Modifying a Vector

To add elements to a vector, we use `push`.

```rust
let mut v: Vec<i32> = Vec::new();

v.push(1);
v.push(2);
v.push(3);
v.push(4);
v.push(5);
```

*Note: when a vector goes out of scope it is dropped along with its contents*

### Reading Elements of a Vector

There are two ways of retrieving a value from a vector. 

Method 1 uses a reference to the original vector which gives us a reference. 

Method 2 uses the `.get(index)` function and a `match` which gives us an `Option<&T>`.

```rust
let v = vec![1, 2, 3, 4, 5];

let third1: &i32 = &v[2];
println!("The third element is {}", third1);

match v.get(2) {
  Some(third2) => println!("The third element is {}", third2),
  None => println!("There is no third element"),
}
```

*Note: vectors are zero-indexed*

### Iterating Over a Vector

Iteration over a vector allows us to access each element successively. 

One way to do this is with a `for` look to get an immutable reference to each element. 

```rust
let v = vec![1, 2, 3, 4, 5];

for i in &vec {
  println!("{}", i);
}
```

We can also iterate over mutable references.

```rust
let mut v = vec![1, 2, 3, 4, 5];

for i in &mut vec {
  let x = *i + 10;
  println!("{}", x);
}
```

### Storing Multiple Types with an Enum

A trick to storing different types inside one vector is to use an enum. As all of the vector elements will be of the enum type, this is valid. 

```rust
enum SpreadsheetCell {
  Int(i64),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadsheetCell::Int(3), 
  SpreadsheetCell::Float(10.12), 
  SpreadsheetCell::Text(String::from("cell"))];
```

## Strings - In Depth

Strings are implemented as a collection of bytes alongside some methods to provide functionality when those bytes are interpreted as text.

Rust has only one string type in its core language, which is the string slice `str`. It can only be handled behind a pointer, so is most commonly seen as `&str`. String literals are stored in the program's binary and therefore are also string slices. 

The `String` type, provided in the standard library, is a growable, mutable, owned, UTF-8 encoded string type. 

*Note: both `str` and `String` are UTF-8 encoded.*

Rust's standard library includes a number of other string types, including:
- `OsString`
- `OsStr`
- `CString`
- `CStr`

The difference between `*String` and `*Str` represents the owned and borrowed types respectively.

Other library crates can provide even more string types. 

### Creating a String

Many of the same operations available for `Vec<T>` are also available for `String`. 

To create a new `String` the `new` function can be used.

```rust
let mut s = String::new();
```

This creates a new, empty string `s` which we can load data into. 

We can also create a `String` from a `str` using the `to_string()` method. This only works because string literals implement the `Display` trait. 

```rust
let data = "initial value"; // String literal

let s = data.to_string(); // String type

let s = "new value".to_string(); // Also works on the literal directly
```

We can also use the `from("...")` method to create a `String` from a string literal.

```rust
let s = String::from("Hello, World!"); // Equivalent to the above code
```

As strings are UTF-8 they can represent an array of different languages. 

### Updating a String

A string can grow in size and its contents can change (like a `Vec<T>`). Either the `+` operator or the `format` macro can be used to concatenate `String` values. 

The `+` operator calls a function with the signature `fn add(self, s: &str) -> String`. 

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("World!");
let s3 = s1 + &s2; // s1 has now been moved to s3
```

*Note: Rust uses [**deref coercion**](#deref-coercion) to allow us to pass a `&String` instead of `&str`.*

If we need to concatenate multiple values, the use of `+` can become unwieldy. Instead we can use the `format` macro.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// Can be replaced with

let s = format!("{}-{}-{}", s1, s2, s3);
```
---
To append to the end of a string we can use `push_str` or `push` if only a character is being appended.

```rust
let mut s = String::from("Hello, ");
s.push_str("World!");

let mut s2 = String::from("lo");
s2.push('l');
```

### Indexing into Strings

In many other programming languages individual characters in a string can be accessed simply by referencing them by index. In Rust, this will cause an error. 

```rust
let s = String::from("Hello, World!");
let h = s[0];
```

The above code will produce the following output.

```rustc
error[E0277]: the trait bound `std::string::String std::ops::Index<{integer}>` is not satisfied
-->
  |
3 | let h = s[0];
  |         ^^^^ the type `std::string::String` cannot be indexed by `{integer}`
  |
  = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
```

To understand why this doesn't work, we must understand the internal representation of a string.

### Internal Representation

A `String` is a wrapper over a `Vec<u8>`.

For single-byte characters, the length of the string is equal to the number of characters; therefore the memory index of each character is simply the position it appears in the string (zero indexed). However, some characters in UTF-8 are not single bytes (one example is the cyrillic alphabet - each character is 2 bytes). 

This makes it trivial as to why simple indexed character retrieval in a string is not possible in Rust. If we had the string `let s = "Здравствуйте"`; would could `s[0]` return? The character 'З' is made up of the bytes 208 and 151. So `s[0]` should return 208, but this alone is not a valid character in UTF-8.

### Bytes, Scalar Values & Grapheme Clusters

For UTF-8, there are three relevant ways for Rust to look at strings. 

For the Hindi string "नमस्ते", it is stored as a `Vec<u8>` that looks like the following.

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

This is just 18 bytes and this how the computer stores the data. 

If we view them as Unicode scalar values (which is what Rust's `char` type is), they make the following array.

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

This appears to be 6 characters, except the fourth and sixth characters aren't actually characters - they're diacritics. 

If we view the data as grapheme clusters, we'd get what a person would call 4 letters.

```rust
["न", "म", "स्", "ते"]
```


This means each program can choose which interpretation of a string that it needs.

A final reason why the indexing into strings is not allowed is because indexing operations are expected to take `O(1)` time. But this cannot be guaranteed with a `String`, because Rust would first have to determine how many valid characters were there.

### Slicing Strings

Because indexing into a string could return several types (bytes, characters, grapheme clusters or a string slice), Rust requires you to be more specific when using indexes. 

To do this, you must specify that you want a string slice by providing a range of indexes. 

```rust
let s = "Здравствуйте";

let ss = &s[0..4];
```

This is perfectly valid syntax to retrieve a string slice. 

Here `ss` will be a `&str`; as each character is 2 bytes (in this example), `ss` will hold the characters `Зд`.

If we tried to pass the indexes `[0..1]`, Rust would panic at runtime. 

```rustc
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside `З` (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4
```

### Iterating Over Strings

If you need to perform operations on individual unicode characters, then you can use the `.chars()` method. 

```rust
for c in "नमस्ते".chars() {
  println!("{}", c);
}
```

This code outputs the following. (Diacritics couldn't be printed individually.)

```
न
म
स

त

```

The `.bytes()` method returns the bytes of each character. 

Getting grapheme clusters is more complicated but crates for it are available.

## Hash Maps

A hash map is a type of content addressable memory where the data itself is (or derives) the key. The means we can achieve O(1) searching, inserting and deleting.

A `HashMap<K, V>` stores a mapping from keys of type `K` to values of type `V`.

It works by using a *hashing function* to place the keys and associated values into memory. 

These can be useful if you want to refer to data not with a numerical index but with a key of any type. 

### Creating a `HashMap`

A hash map can be created using the `new` method. 

The below example creates a `HashMap<String, i32>` for two teams, Blue and Yellow, which start with 10 and 50 points respectively. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Hash maps store their data on the heap. And like vectors, they are homogeneous (all keys must be the same type and all the values must be the same type). 

Another way of creating a hash map is by using the `collect` method on a vector of tuples, where each tuple consists of a key and a value. The `collect` method gathers data into a number of collection types.

The below example creates the hash map for the Blue and Yellow teams, which are stored in two separate vectors. The `zip` method creates a vector of tuples from the original two vectors. 

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let scores = vec![10, 50];

let team_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
```

*Note: type annotation is required here as `collect` can be used to create many different data structures. However, Rust can still infer the types of `K` and `V` so we can use `_` in the annotation.*

### Hash Map Ownership

If a type implements the `Copy` trait (e.g. `i32`), the values are copied into the hash map. For types that don't implement the `Copy` trait, the values are moved into the hash map and the hash map will be the owner. 

### Accessing Values

We can retrieve the value from a hash map by passing the key into it. 

```rust
use std::collections::HashMaps;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

Here `score` will have the value that is associated with the key `Blue`. The result will be `Some(&10)`. 

The result is an `Option<&V>`; if there is no value for the given key it will return `None`. 

We can also iterate over the key-value pairs. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
  println!("{}, {}", key, value);
}
```

### Updating a Hash Map

As each key can only be associated with one value, values are often changed. 

To overwrite a value, we simply use `insert` again with the key whose value you want to overwrite and the new value. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

scores.insert(String::from("Blue"), 108); // Replaces 10 with 108
```

---

We can also write a new value, only if there was not a previous value associated with that key. Rust has a method for this called `entry`. It takes the key as a parameter and returns an `Entry` enum which represents a value that might or might not exist. 

For example, we want to write the value `50` into `Yellow`, only if there isn't a value already associated with the key `Yellow`. And the same for the Blue team with the score `10`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(10);
```

The `or_insert` method on `Entry` is defined to return a mutable reference to the value of the corresponding value for the given key if it exists. If not, the value is inserted and a mutable reference to this new value is returned. 

---

Another option is to update values depending on the value that is already there. 

For example, if we want to count how many times a word occurs in a string, we can have the word as the key in the hash map and the count as the value.

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

let text = String::from("hello world wonderful world");

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}
```

# Error Handling

To ensure reliability, Rust often requires you to acknowledge and handle any possible errors before the code can compile. 

Rust groups all errors into two self-explanatory categories: **recoverable**, and **unrecoverable**.

Rust doesn't use exceptions (like other languages), instead it returns a `Result<T, E>` for recoverable errors and the `panic!` macro which stops execution (and calls destructors) for unrecoverable errors.

## Unrecoverable Errors with `panic!`

When the `panic!` macro runs, it will display an error message, unwind and clean up the stack and then exit. 

*Note: if you want the program to abort instead of unwind, you can add the following into the `Cargo.toml`*

```
[profile.release]
panic = 'abort'
```

:s
If we call `panic!("crash")`, the following output would occur at runtime.

```
thread 'main' panicked at 'crash', std/main.rs:2:5
note: Run with 'RUST_BACKTRACE=1' for a backtrace
```

### Using a `panic!` Backtrace

The output from the macro looks slightly different if it is called from a library that we have not written ourselves. This can be confusing if we don't expect `panic` to be called. 

```rust
fn main() {
  let v = vec![1, 2, 3];

  v[99];
}
```

At first glance, this code does not call the `panic!` macro. However, if we try to run it we get the following output. 

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /libcore/slice/mod.rs:2448:10
note: Run with 'RUST_BACKTRACE=1' for a backtrace
```

This error points to a file that we did not write. In this case, it points to the implementation of `slice`. 

The next line tells us how we can enable a backtrace. A backtrace shows a list of all functions that have been called to get to that point. Like other languages, Rust's backtraces start from the newest call, and so the key to reading them is by reading them is to start from the top and read until you reach a file you wrote. That's where the problem has arisen. 

We can run with the backtrace enabled by setting the `RUST_BACKTRACE` environment variable to a non-zero value.

```
RUST_BACKTRACE=1 cargo run
```

We can also run with `RUST_BACKTRACE=full` for a more verbose backtrace.

Example backtraces are shown below.

```
> RUST_BACKTRACE=1 cargo run
warning: unused variable: `val`
 --> src/main.rs:9:9
  |
9 |     for val in args {
  |         ^^^ help: if this is intentional, prefix it with an underscore: `_val`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `sorter` (bin "sorter") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sorter`
thread 'main' panicked at 'No arguments passed - nothing to sort!', src/main.rs:7:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/panicking.rs:101:14
   2: sorter::main
             at ./src/main.rs:7:9
   3: core::ops::function::FnOnce::call_once
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

```
> RUST_BACKTRACE=full cargo run
warning: unused variable: `val`
 --> src/main.rs:9:9
  |
9 |     for val in args {
  |         ^^^ help: if this is intentional, prefix it with an underscore: `_val`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `sorter` (bin "sorter") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sorter`
thread 'main' panicked at 'No arguments passed - nothing to sort!', src/main.rs:7:9
stack backtrace:
   0:     0x55f876912e6c - std::backtrace_rs::backtrace::libunwind::trace::h3fea1eb2e0ba2ac9
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x55f876912e6c - std::backtrace_rs::backtrace::trace_unsynchronized::h849d83492cbc0d59
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x55f876912e6c - std::sys_common::backtrace::_print_fmt::he3179d37290f23d3
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x55f876912e6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h140f6925cad14324
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x55f87692d79c - core::fmt::write::h31b9cd1bedd7ea38
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/fmt/mod.rs:1150:17
   5:     0x55f876911015 - std::io::Write::write_fmt::h1fdf66f83f70913e
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/io/mod.rs:1667:15
   6:     0x55f876914560 - std::sys_common::backtrace::_print::he7ac492cd19c3189
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x55f876914560 - std::sys_common::backtrace::print::hba20f8920229d8e8
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x55f876914560 - std::panicking::default_hook::{{closure}}::h714d63979ae18678
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:210:50
   9:     0x55f876914117 - std::panicking::default_hook::hf1afb64e69563ca8
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:227:9
  10:     0x55f876914c14 - std::panicking::rust_panic_with_hook::h02231a501e274a13
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:624:17
  11:     0x55f8769146c2 - std::panicking::begin_panic_handler::{{closure}}::h5b5e738edf361af7
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:519:13
  12:     0x55f876913334 - std::sys_common::backtrace::__rust_end_short_backtrace::h601a115def7987b1
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:141:18
  13:     0x55f876914659 - rust_begin_unwind
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:517:5
  14:     0x55f8768f7da1 - core::panicking::panic_fmt::h7a58c8fffc5559a4
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/panicking.rs:101:14
  15:     0x55f8768fb1f9 - sorter::main::h7eb128ef6458e954
                               at /home/tg/Code/learningRust/sorter/src/main.rs:7:9
  16:     0x55f8768f9aab - core::ops::function::FnOnce::call_once::ha2575617c76d81c3
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ops/function.rs:227:5
  17:     0x55f8768f96ae - std::sys_common::backtrace::__rust_begin_short_backtrace::h8e6f3cd03635da24
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/sys_common/backtrace.rs:125:18
  18:     0x55f8768fdd91 - std::rt::lang_start::{{closure}}::hacac2e433ff5a817
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/rt.rs:63:18
  19:     0x55f87691511a - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h2790017aba790142
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ops/function.rs:259:13
  20:     0x55f87691511a - std::panicking::try::do_call::hd5d0fbb7d2d2d85d
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:403:40
  21:     0x55f87691511a - std::panicking::try::h675520ee37b0fdf7
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:367:19
  22:     0x55f87691511a - std::panic::catch_unwind::h803430ea0284ce79
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panic.rs:129:14
  23:     0x55f87691511a - std::rt::lang_start_internal::{{closure}}::h3a398a8154de3106
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/rt.rs:45:48
  24:     0x55f87691511a - std::panicking::try::do_call::hf60f106700df94b2
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:403:40
  25:     0x55f87691511a - std::panicking::try::hb2022d2bc87a9867
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:367:19
  26:     0x55f87691511a - std::panic::catch_unwind::hbf801c9d61f0c2fb
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panic.rs:129:14
  27:     0x55f87691511a - std::rt::lang_start_internal::hdd488b91dc742b96
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/rt.rs:45:20
  28:     0x55f8768fdd60 - std::rt::lang_start::h684a3acc2f36a4ed
                               at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/rt.rs:62:5
  29:     0x55f8768fb23c - main
  30:     0x7f0269e9c2d0 - <unknown>
  31:     0x7f0269e9c38a - __libc_start_main
  32:     0x55f8768f84f5 - _start
                               at /build/glibc/src/glibc/csu/../sysdeps/x86_64/start.S:115
  33:                0x0 - <unknown>
```

They can look confusing, but once you get used to reading them you will be comfortable with them and you'll understand how useful they are to help debug your program. 

*Note: backtraces require debug symbols to be enabled. This is default behaviour for `cargo run` and `cargo build` when run **without** the `--release` flag.*

## Recoverable Errors with `Result`

Most errors aren't severe enough to require a complete termination of execution. For example, if you try to open a file and it fails because the file doesn't exist, you may just want to create the file instead. 

The `Return` enum has two variants.

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

`T` represents the type of the value returned in a successful case. `E` represents the type of error returned in a failed case.

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt");
}
```

Because this operation may fail, it returns a `Result<std::fs::File, std::io::Error>`. This means if the operation succeeds then a file handle is returned; if it fails, an IO error occurs. 

We can retrieve the values in the enum using a `match`.

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
}
```

### Matching Different Errors

It is often useful to perform different actions depending on which type of error occurs. 

For example, if the previous example failed because the file doesn't exist, we may want to create the file. If it fails for any other reason, we will panic the same way as before. 

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.match_kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file, {:?}", e),
      },
      other_error => panic!("Problem opening the file, {:?}", other_error),
    },
  };
}
```

This may look confusing, so we will break it down. The first match operation checks for the result of the `File::open("hello.txt")` method. If this method returns `Ok(file)` the file handler is returned. If an error occurred, another match operation is used. This checks whether the type of `error` (from the first match statement), is equal to `ErrorKind::NotFound`; if it is equal, the file is created and the file handler is returned if the creation is successful (and a panic if an error occurs at this stage). If it is not that type of error, the code panics and outputs the `other_error` which occurred during the opening of the file.

This code can be made more concise. 

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File.open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file; {:?}", error);
      })
    } else {
      panic!("Error opening the file; {:?}", error);
    }
  });
}
```

This code doesn't use any `match` statements. 

### Shortcuts for Panic on Error

Sometimes using match statements can become confusing and/or tedious to write. The `Result<T, E>` enum has several methods to help with this. 

One such method is `unwrap`. If the result is `Ok`, the value is returned. If not, the panic macro is called. 

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt").unwrap();
}
```

There is a related method which allows us to write the message passed to the `panic` macro. This is the `expect` method. 

```rust
use std::fs::File;

fn main() {
  let f = File::open("Hello.txt").expect("Failed to open Hello.txt");
}
```

This works the same way as `unwrap`; the only difference is the custom panic macro text. 

### Propagating Errors 

It is often useful to handle errors in the calling code and not in the method itself. 

The following code reads a username from a file. If the file doesn't exist or cannot be opened the error is propagated to the calling code. 

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("name.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e);
  }

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
```

Here the return type of the function is a `Result`. This allows the error to be passed to the calling code, otherwise the username will be returned. If an error occurs when opening the file, the error is returned (in the first match). If this succeeds, we create a new, empty string. We need another match statement in case the read method fails. The result of the match is returned, regardless of whether the operation is successful or not. 

This is so common for error propagation that Rust provides the `?` operator. 

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("name.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
```

The `?` operator can only be used on methods that return `Result`. If the value returned is `Ok`, the associated value will be returned from that method. If an error occurs, this error will be returned for the whole outer function. 

This operator is slightly different from the `match` expressions. Error values that have it called on them will be converted into the type that is returned from the outer function. This is because they go through the `from` which is defined in the `From` trait, which converts errors into different types of errors. 

The above code can be condensed even further by chaining the methods.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
```

Reading from a file into a string is a common operation so Rust has a method for it in the standard library.

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
```

The `?` operator can only be used on methods which return a `Result` as it is defined using a `match` on the `Result` enum and in methods that return a `Result`. Therefore, we cannot use `?` in `main` as it returns `()`. 

We can return a `Result` from the main function, but only for a specific `Result`. The return result enum must be the following: `Result<(), Box<dyn Error>>`. The `Box<dyn Error>` is a trait object (which will be covered later). 

### When to `panic!`

When code panics, there's no way to recover. If you return a `Result`, you give the calling code options instead of making the decision to panic. Therefore, returning a `Result` is a good default option for your code. Sometimes it is more appropriate to panic instead of returning a `Result`, such as in tests and in prototype code. 

