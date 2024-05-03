<b> [<<Home](../Readme.md) </b>

# Types of assignments

## Immutable variables
By default, variables are immutable in Rust. You can make them mutable by using the `mut` keyword.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This will throw an error
    println!("The value of x is: {}", x);
}
```

## Mutable variables
You can make variables mutable by using the `mut` keyword.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This will not throw an error
    println!("The value of x is: {}", x);
}
```

## Shadowing
You can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable.\
This is useful when you want to change the type of a variable but keep the same name.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

## Constants 
Constants are always immutable. You must specify the type of the constant.

```rust
const MAX_POINTS: u32 = 100_000;
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

- Integers : Signed and unsigned integers. Signed integers can be positive, negative, or zero. Unsigned integers can only be positive or zero.
    - Signed integers   - i8, i16, i32, i64, i128, isize
    - Unsigned integers - u8, u16, u32, u64, u128, usize

    the range of `i8 is -128 to 127` and for `u8 is 0 to 255` and length of integer is 8 bits ie `2^8 = 256`

- Floating-point numbers : Rust has two primitive types for floating-point numbers: 
    - f32 and f64.
- Booleans : Represents logical values. Booleans can either be true or false.
    - bool
- Characters : Represents a single Unicode character. Rust’s char type is four bytes in size and represents a Unicode Scalar Value.
    - char


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

- Tuples : A tuple is a collection of values of different types. Once declared, the values of a tuple cannot be changed.
- Arrays : An array is a collection of values of the same type. Arrays in Rust have a fixed length, which cannot be changed.

```rust
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let arr: [i32; 5] = [1, 2, 3, 4, 5];

        // we can define with or without type
        let tup = (500, 6.4, 1);
        let arr = [1, 2, 3, 4, 5];

        println!("The value of x is: {}", tup.0);
        println!("The value of x is: {}", arr[0]);
    }
```
### mutability of compound types

```rust
    fn main() {
        let mut tup: (i32, f64, u8) = (500, 6.4, 1);
        let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

        tup.0 = 100;
        arr[0] = 100;
    }
```

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