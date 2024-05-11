<b> [<<Home](../Readme.md) </b>


- [Enumerations \& Pattern Matching](#enumerations--pattern-matching)
    - [Anoymous Structs](#anoymous-structs)
    - [Generic and Non-Generic Enums](#generic-and-non-generic-enums)
  - [Methods](#methods)
  - [The Option Enum](#the-option-enum)
    - [Arethmetic with `Option<T>`](#arethmetic-with-optiont)
  - [The `match` Control Flow Operator](#the-match-control-flow-operator)
  - [Patterns That Bind to Values](#patterns-that-bind-to-values)
  - [Matching with `Option<T>`](#matching-with-optiont)
  - [The `_` Placeholder](#the-_-placeholder)
  - [Concise Control Flow (`if` Statements)](#concise-control-flow-if-statements)


# Enumerations & Pattern Matching

An enumeration (A.K.A. `enum`) allows you to create a type by defining all of its possible values. An instance of an `enum` can only take **`one value at a time`**.

> enums are very similar to structs, but with a few key differences. Key differences include the fact that you can only have `one value at a time`, and that the value `can be one of several variants`.

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



### Anoymous Structs

An `enum` can also be used to create an anonymous struct. This is useful when the data is only relevant to the `enum` and not to the program as a whole.

```rust

enum Message {
  Quit, // variant with no data
  Move { x: i32, y: i32 }, // anonymous struct
  Write(String), // variant with a single String
  ChangeColor(i32, i32, i32), // variant with three i32s
}

fn main() {

    let m = Message::Quit;
    let m = Message::Write(String::from("hello"));
    let m = Message::Move { x: 10, y: 20 };
    let m = Message::ChangeColor(0, 160, 255);
}

```
### Generic and Non-Generic Enums 

Enums can also be generic. This allows the `enum` to take any type as a parameter. 

<u>**Generic Enums**</u>

```rust
enum Option<T> {
  Some(T),
  None,
}
```

``` rust
let some_number = Some(5);
let some_string = Some("a string");

let some_number : Option<i32> = Option::Some(5);
let some_string : Option<&str> = Option::Some("a string");


// accessing the value

let number = some_number.unwrap();

```

> Genric Enums cannot be combimed with non-generic enums.


## Methods 

Methods can also be defined on an `enum` using an `impl` block. These methods also take `self` or references to `self` as the first parameter (unless it's an associated function) and can be called using the dot notation (`enumExample.method()`).

```rust



enum IpAddress {
  V4(u8, u8, u8, u8),
  V6(String),
}

impl IpAddress {
  fn print(&self) {
    match self {
      IpAddress::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
      IpAddress::V6(s) => println!("{}", s),
    }
  }
}

fn main() {
  let v4 = IpAddress::V4(127, 0, 0, 1);
  let v6 = IpAddress::V6(String::from("::1"));

  v4.print();
  v6.print();
}

```



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


> Optional is so useful that it is included in the prelude, so you don't need to bring it into scope explicitly. In addition, so are its variants: you can use `Some` and `None` directly without the `Option::` prefix.


```rust

let some_number = Some(5);
let some_string = Some("a string");

let some_number : Option<i32> = Some(5);
let some_string : Option<&str> = Some("a string");

let absent_number: Option<i32> = None;


```


### Arethmetic with `Option<T>`

The `Option<T>` enum can be used to perform arithmetic operations on values that may be `None`. 

```rust

let a = Some(5);
let b = 10;

let c = a+b; // This will not compile as `a` is an `Option<i32>`

let c = b + a.unwrap(); // This will compile, but will panic if `a` is `None

let c = b + a.unwrap_or(0); // This will compile and will not panic if `a` is `None` as it will default to 0

```


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
