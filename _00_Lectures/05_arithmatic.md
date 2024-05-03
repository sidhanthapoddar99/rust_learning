<b> [<<Home](../Readme.md) </b>


# Basic operations

```rust

fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}

```

# Convert string to number

```rust

fn main() {
    let x = "5".parse().unwrap();
    let y: i32 = "10".parse().unwrap();
    let z: f64 = "5.2".parse().unwrap();

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}

```

# String operations

- `parse` : Convert string to number
- `to_string` : Convert number to string
- `push_str` : Append a string to another string
- `format` : Format a string
- `+` : Concatenate two strings
- `&`: Concatenate two strings
- `println!` : Print a string
- `trim` : Remove leading and trailing whitespaces
- `len` : Get the length of a string
- `is_empty` : Check if a string is empty
- `split_whitespace` : Split a string by whitespace
- `chars` : Iterate over the characters of a string
- `lines` : Iterate over the lines of a string


```rust

fn main() {
    let mut hello = String::from("Hello, ");
    let world = "world!";

    hello.push_str(world);

    println!("{}", hello);
}

```

## String concatenation

```rust

fn main() {
    let hello = "Hello, ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;

    println!("{}", hello_world);
}

``` 

## String formatting

```rust

fn main() {
    let name = "John";
    let age = 30;

    let greeting = format!("My name is {} and I am {}", name, age);

    println!("{}", greeting);
}

```