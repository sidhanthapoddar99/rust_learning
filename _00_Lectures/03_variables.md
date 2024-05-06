<b> [<<Home](../Readme.md) </b>

# Types of assignments

## Immutable variables
By default, variables are immutable in Rust. You can make them mutable by using the `mut` keyword.

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

## Mutable variables
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

## Shadowing

A new variable can be declared with the same name as the original. We say the original variable is **shadowed** by the new variable. `let` must be used when shadowing.

This is useful when you want to change the type of a variable but keep the same name.


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

## Constants 
Constants can be declared using the `const` keyword instead of `let`. For constants, the type must be explicitly stated.\
*Note: `mut` cannot be used with constants*\
```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```


## **Shadowing vs. mut**: Shadowing is different from marking a variable as mutable.\

Mutable variable(`mut`): When you declare a variable as mutable using the mut keyword, you can change the value of the variable, but not its type. This is useful when you want to modify the value of a variable over time.

```rust
fn main() {
    let mut x = 5;
    x = 6;
}
```
Shadowing:  When you shadow a variable, you're actually creating a new variable with the same name. This new variable can have a different type. The original variable remains unchanged and is simply hidden (or "shadowed") by the new variable.


```rust
fn main() {
    let x = "   ";
    let x = 4;
}
```

# Scope
The scope of a variable is the range within a program for which the variable is valid.\
Rust has block-scoped variables.\
A variable is valid from the point at which it is declared until the end of the block in which it is declared.


## Defing own scope

```rust
fn main() {
    let x = 5; // x is valid from this point forward
    
    {
        let y = 10; // y is valid from this point forward
        println!("x: {}, y: {}", x, y);

    } 
    
    // y is no longer valid
    println!("x: {}", x);
}

// x and y are no longer valid
```


## Exterior and interior scopes

```rust

fn main() {
    let x = 5; // x is valid from this point forward
    
    println!("x: {}", x);

    {
        let x = 10; // y is valid from this point forward
        println!("x: {}", x);

    } 

    let x = x + 1;

    println!("x: {}", x);
    
    // output will be 5, 10, 6
    // x assigned to 5
    // x assigned to 10 in interior scope
    // x assigned to x + 1 but for exterior scope x is still 5 as we have redefined x in interior scope
}

```

One more exampel of using interior and exterior scope combined 

```rust

fn main() {
    let x = 5; // x is valid from this point forward
    
    println!("x: {}", x);

    {
        let x = x + 2; // y is valid from this point forward
        println!("x: {}", x);

    } 

    let x = x + 1;

    println!("x: {}", x);
    
    // output will be 5, 7, 6
    // x assigned to 5
    // x assigned to 7 in interior scope, first x is used from exterior scope and then added 2 to it and assigned to x which is in interior scope so x_interior = x_exterior + 2
    // x assigned to x + 1 but for exterior scope x is still 5 as we have redefined x in interior scope
}

```

# Data Types

There are two types of data types in Rust:
1. Scalar : Represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
2. Compound : Represents a group of multiple values. Rust has two primitive compound types: tuples and arrays.

## Scalar Types

### 1. Integers 
 
 Signed and unsigned integers. Signed integers can be positive, negative, or zero. Unsigned integers can only be positive or zero.

Signed integers are in the format `ixxx` and unsigned are in the format `uxxx`, where `xxx` is the number of bits.

- Signed integers   - i8, i16, i32, i64, i128, isize
- Unsigned integers - u8, u16, u32, u64, u128, usize

the range of `i8 is -128 to 127` and for `u8 is 0 to 255` and length of integer is 8 bits ie `2^8 = 256`

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

### 2. Floating-point numbers

Rust has two primitive floating-point numbers, `f32` and `f64`. The default type is `f64` as on modern CPUs it is the same speed as `f32` but with higher precision.

### 3. Booleans

Represents logical values. Booleans can either be true or false.
    - `bool`

### 4. Characters 

Represents a single Unicode character. Rust’s char type is four bytes in size and represents a Unicode Scalar Value.
    - `char`


### Examples

how to define all these varibales

```rust
    fn main() {
        let x: i8 = 5;
        let y: u8 = 5;
        let z: f32 = 5.0;
        let a: f64 = 5.0;
        let b: bool = true;
        let c: char = 'a';
    }
```

## Compound Types


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
  let (x, y, z) = tup; // x = 500, y = 6.4, z = 1 // destructuring

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


### how to initialize and empty array and tuple

In rust we dont intialize large homogeneous arrays with empty values, we can use `vec` for that.\
For tuple we need to manually initialize all the value and cannot do auto initrialization like in python.

```rust
    fn main() {
        let arr = [3; 5]; // [3, 3, 3, 3, 3]
    }
```

## Vector

Vectors are similar to arrays, but they can grow or shrink in size.\
Vectors are defined using the `Vec<T>` type.\
Vectors are stored on the heap, not the stack.\
To create a new, empty vector, you can call the `Vec::new` function.

```rust
    fn main() {
        let v: Vec<i32> = Vec::new();

        let v: Vec<i32> = vec![1, 2, 3];
    }
```

## String

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.\
The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.\
Rust strings are stored on the heap, not the stack.\
To create a new, empty string, you can call the `String::new` function.

```rust
    fn main() {
        let s: String = String::new();

        let s = String::from("hello");
    }
```

## Type Casting

Rust is a statically typed language, which means that it must know the types of all variables at compile time.\

Rust provides the `as` keyword to convert data types from one type to another.\

```rust
    fn main() {
        let x: i32 = 5;
        let y: f64 = x as f64;
    }
```






