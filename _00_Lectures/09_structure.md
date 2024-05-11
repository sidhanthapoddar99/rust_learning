<b> [<<Home](../Readme.md) </b>


- [Structure Related Data](#structure-related-data)
  - [Building a `struct`](#building-a-struct)
  - [Nested Structs](#nested-structs)
  - [Inheritance](#inheritance)
  - [reusing struct data | `struct` Update Syntax](#reusing-struct-data--struct-update-syntax)
  - [Tuple Structs](#tuple-structs)
  - [Unit-like Structs](#unit-like-structs)
  - [Methods](#methods)
    - [Defining Methods](#defining-methods)
    - [Associated Functions](#associated-functions)
  - [Variable Types in Structs](#variable-types-in-structs)


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

> Structure are just like other variables they can be `mutable` or `immutable` based on the declaration.

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



## Nested Structs

Structs can be nested within each other.

```rust
struct User {
  username: String,
  password: String, 
  age: u16,
  loggedIn: bool,
}

struct Admin {
  user: User,
  permissions: Vec<String>,
}
```

## Inheritance

Inheritance can be simulated by using a struct within another struct.

```rust
struct User {
  username: String,
  password: String, 
  age: u16,
  loggedIn: bool,
}

struct Admin {
  user: User,
  permissions: Vec<String>,
}
```



## reusing struct data | `struct` Update Syntax 

If a struct contains only `Copy` types, it can be copied by using the `Copy` trait.

```rust
struct User {
  username: String,
  password: String, 
  age: u16,
  loggedIn: bool,
}

fn main() {
    let user1 = User {
        username: String::from("name"),
        password: String::from("password"),
        age: 10,
        loggedIn: false,
    };
    
    let user2 =  User{ 
        username: user1.username,
        password: user1.password,
        ..user1
    };
}
```

This is useful when you want to make a new struct based on an old one but with some values changed. 

This syntax will auto-fill any fields that have not been redefined with their values from the older struct. 

## Tuple Structs

Tuple structs look like normal tuples but benefit from the name of the struct. 

```rust
struct Colour(i32, i32, i32);
struct Point(i64, i64, i64, i64);

let black = Colour(0, 0, 0);
let origin = Point(0, 0, 0);

// Accessing the fields
let black_r = black.0;
let origin_x = origin.0;

```

`Point` and `Colour` are still different structs even though they have the same fields. Other than this, these structs behave like tuples. 

> This is different from a tuple because the type is different. ie `Color` is not the same as `(u8, u8, u8)` even though they have the same elements. 

> Eg we cannot pass a tuple `(u8, u8, u8)` to a function that expects a `Color` and vice versa. 

## Unit-like Structs

Unit-like structs have no fields (hence the name; they are similar to the unit type - `()`). These can be useful if you want to implement a trait but have no data for it.

## Methods

Methods are simply functions defined inside of a `struct`. Their first parameter is always `self` which references the instance of the struct itself upon which the method will act. 

### Defining Methods

> To add a method to a struct we first need an `impl` (implement) block for the struct. Inside this block define a method the same as a function but ensuring the first parameter   `&self`.

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



## Variable Types in Structs

Rust allows for generic types in structs.

```rust

struct User <T1, T2> // T1 and T2 are generic types
{
  Name: String,
  Age: u16,
  Data: T1, // T1 is a generic type
  MoreData: T2, // T2 is a generic type
}

impl User <String, i32> // T1 is a String and T2 is an i32
{
  // This method is only available to User <String, i32>
  // Associated function that creates a new User
  fn new(name: String, age: u16, data: String, moreData: i32) -> User <String, i32>
  {
    User
    {
      Name: name,
      Age: age,
      Data: data,
      MoreData: moreData,
    }
  }

}

fn main()
{
  let user1 = User::new(String::from("name"), 10, String::from("data"), 100);
  // user1 is of type User <String, i32> and we are using the associated function new to create it

  let user2 = User::<i32, i32> {
    Name: String::from("name"),
    Age: 10,
    Data: 100,
    MoreData: 100,
  }
  // user2 is of type User <i32, i32> and we are defining it directly
}

```
