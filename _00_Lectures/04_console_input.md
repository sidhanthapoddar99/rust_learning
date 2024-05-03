<b> [<<Home](../Readme.md) </b>

# Console Input

```rust
use std::io;

fn main() {
    println!("Enter a name");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    println!("Hello, {}", name);
}

```
- here `name` is a mutable variable of type `String`.
- `& name` is a reference to the variable `name`.
- `&mut name` is a mutable reference to the variable `name`. This allows us to modify the variable `name`.
- `io::stdin().read_line(&mut name)` reads the input from the user and stores it in the variable `name`.
