
- [Packages, Crates \& Modules](#packages-crates--modules)
  - [Terminologies](#terminologies)
  - [Packages \& Crates](#packages--crates)
  - [Defining Modules](#defining-modules)
    - [Module Example - Restaurant Library](#module-example---restaurant-library)
    - [Referring to a Module](#referring-to-a-module)
  - [Making Structs \& Enums Public](#making-structs--enums-public)
  - [Bringing Paths into Scope](#bringing-paths-into-scope)
    - [Conflicting Names](#conflicting-names)
  - [Using External Packages](#using-external-packages)
    - [Nested Paths](#nested-paths)
    - [The `glob` Operator](#the-glob-operator)
  - [Separating Modules into Different Files](#separating-modules-into-different-files)


# Packages, Crates & Modules

Rust and Cargo have many features to help manage larger projects. 

**Packages:** A Cargo feature that lets you build, test and share crates

**Crates:** A tree of modules that produce a library or an executable

**Modules:** Let you control the organisation, privacy and scope of paths

**Paths:** A way of naming items


## Terminologies

When we do `cargo new project_name`, we are creating a `package`.

A `package` can contain multiple `crates`. `caretes` are either a `library` or an `executable`.
- A `library` is a collection of functions that can be `used in other programs`.
- An `executable` is a program that can be `run on its own`

`crates` can contain multiple `modules`. `modules` are a way to `organize code` and `control the privacy of paths`.

<u>Example:</u> 
- we have a authentication `library`, 
- we can have a `crate` for `authentication` 
- `modules` for `login`, `register`, `check_user` etc.
- we can make `login` and `register` public and `check_user` private.

if we want to call the `login` function from the `authentication` crate, we can have to specify the path to the `login` function. 

structure of the `authentication crate` in `User Package`:

```
User Package
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ authentication
        |__ mod.rs
        |__ login.rs
        |__ register.rs
        |__ check_user.rs
```

In the `mod.rs` file, we can specify the path to the `login` function. 
```rust
// mod.rs
pub mod login;
pub mod register;
mod check_user;
```

In the `lib.rs` file, we can call the `login` function. 
```rust
// lib.rs
mod authentication;
use authentication::login;
```
`Cargo.toml` file will have the following dependencies. 
```toml
[dependencies]
```

## Packages & Crates

The **crate root** is a source file that the Rust compiler starts from and makes up the root module of the project. 

A package is one or more crates that provides functionality. A package contains a `Cargo.toml` file which describes how to build the package.
- A package must contain zero or one library crate 
- as many binary crates as desired. 
- But it must contain at least one crate. 

> Note: all the functionality of a crate is defined within the crate's namespace

- When `main.rs` is present in the source directory, a `binary crate` with the <u> same name as the `package` </u> is created automatically.
  - `main.rs` is the `crate root` of the binary crate.
- When `lib.rs` is present in the source directory, a `library crate` with the <u> same name as the `package` </u> is created automatically
  - `lib.rs` is the `crate root` of the library crate.
- `binary crate` is used to create an executable, while `library crate` is used to create a library.
- so even though we dont define crate in the `Cargo.toml` file, it is created automatically if `main.rs` or `lib.rs` is present in the source directory.

<u> Example: </u> 

`Library Crate` with `lib.rs` as the crate root
```
User Package
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ authentication
        |__ mod.rs
        |__ login.rs
        |__ register.rs
        |__ check_user.rs
```

`Binary Crate` with `main.rs` as the crate root
```
User Package
|__ Cargo.toml
|__ src
    |__ main.rs
    |__ authentication
        |__ mod.rs
        |__ login.rs
        |__ register.rs
        |__ check_user.rs
```

Module with 1 `library crate` and 1 `binary crate`
```
User Package
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ main.rs
```
> here `lib.rs` is the crate root of the `library crate` and `main.rs` is the crate root of the `binary crate`.


Module with 1 `library crate` and multiple `binary crate`
```
User Package
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ bin
        |__ another_binary.rs
        |__ yet_another_binary.rs
```

> here `lib.rs` is the crate root of the `library crate` and each file in the `bin` represents a `binary crate`.

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

<u> Module Tree: </u>
```
restaurant
 crate
      |__ front_of_house
          |__ hosting
          |   |__ add_to_waitlist
          |   |__ seat_at_table
          |__ serving
              |__ take_order
              |__ serve_order
              |__ take_payment
```

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

  pub mod serving {
    pub fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
  }
}

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();

    // Relative path
    self::front_of_house::serving::take_order();

}
```

The function `eat_at_restaurant()` is now defined within the crate's root. It uses the `pub` keyword to expose it to the crate's public API.

However, this code will fail to compile due to Rust's privacy boundaries. In Rust, everything  is private by default. A method cannot use a child item's private item, but can use an ancestor's private items. 

For our example to compile, we need to add `pub` to both `hosting` and the methods inside of it. We do not need to add `pub` to `front_of_house` because it is defined inside the same module as `eat_at_restaurant()`.

## Making Structs & Enums Public

The process of making structs and enums public is similar to that of methods but with some extra details. 

If we use `pub` before the struct definition, the struct will be public but the fields will still be private. We can then denote which fields inside the struct should be public. If one or more of the fields are still private, then an associated function constructor must exist otherwise no instances of the struct could ever be created. 

If you add `pub` before the definition of an enum, all values inside the enum will be public. 

Example:

```rust
mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant(){
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
}
```


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
> Note: `use` keyword is used to bring the path into scope.
> In the above example, `use crate::front_of_house::hosting;` is used to bring the `hosting` module into scope.
> This allows us to use `hosting::add_to_waitlist();` and `hosting::seat_at_table();` without having to write the full path.


We can also provide these modules new names using the `as` keyword.

```rust
use std::io::Result as IoResult;
```

These names can be `re-exported` to allow external code to call items brought into scope. To do this, we can use `pub use`.

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist(){}
    pub fn seat_at_table(){}
  }
}

pub use crate::front_of_house::hosting;
```


### Conflicting Names

If two items have the same name, we can either bring the parent module into scope or rename the item. 

```rust
// Bringing the parent module into scope
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
  // code
}

fn function2() -> io::Result<()> {
  // code
}
```

or 

```rust
// Renaming the item
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn function1() -> FmtResult {
  // code
}

fn function2() -> IoResult<()> {
  // code
}
```




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

use rand::{Rng, thread_rng, CryptoRng};
```

`self` can also be used in nested paths.

```rust
use std::io;
use std::io::Write;

// Can be replaced with

use std::io::{self, Write};
```

### The `glob` Operator

We can bring `all public items` defined in a path into scope using the `glob` operator, `*`.

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

<u> Example: </u>

Previously, we had the following structure for `lib.rs` 

```
restaurant
|__ Cargo.toml
|__ src
    |__ lib.rs
```

```rust
// lib.rs
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
```
---
If we move the `front_of_house` module into a separate file `front_of_house.rs`, the structure would look like this. 

```
restaurant
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ front_of_house.rs
```

```rust
// lib.rs
mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant(){
  hosting::add_to_waitlist();
  hosting::seat_at_table();
}
```

```rust
// front_of_house.rs
pub mod hosting {
  pub fn add_to_waitlist(){}
  pub fn seat_at_table(){}
}
```

---


If we move the `hosting` module into a separate file `hosting.rs`, the structure would look like this. 
```
restaurant
|__ Cargo.toml
|__ src
    |__ lib.rs
    |__ front_of_house
        |__ mod.rs
        |__ hosting.rs
        |__ serving.rs
```

```rust
// lib.rs
mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant(){
  hosting::add_to_waitlist();
  hosting::seat_at_table();
}
```

```rust
// front_of_house/hosting.rs
pub fn add_to_waitlist(){}
pub fn seat_at_table(){}
```
