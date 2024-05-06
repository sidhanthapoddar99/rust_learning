
<b> [<<Home](../Readme.md) </b>

# Static Typing vs Dynamic Typing

- **Static Typing**: The type of a variable is known at compile time. Eg: C, C++, Rust, Java, TypeScript
- **Dynamic Typing**: The type of a variable is known at runtime. Eg: Python, Ruby, JavaScript




# Basic of rust

- staticlly and strong typed language
- compile to machine code like C
- Extermely fast and reliable
- High level features like python but low level control like C
- Memory safe
- No garbage collector
- No null pointer exception

# What can we make with rust
- command line tools
- web server
- web assembly
- embedded system
- game
- blockchain
- bioinformatics
- machine learning
- eg Firefox, Dropbox, Cloudflare, etc

# Resources
1. Rust Book: https://doc.rust-lang.org/book/
2. Rust Install Docs: https://doc.rust-lang.org/book/ch01-01-installation.html
3. Rust Installer Download Page: https://www.rust-lang.org/tools/install
4. Learning https://www.youtube.com/playlist?list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ
5. Playlist https://www.youtube.com/playlist?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL
6. Therory https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8

# Basic Syntax

- `.rs` is the file extension for Rust source files
- `fn main() { }` is the entry point of the program
- `println!("Hello, world!");` is used to print to the console
- `let x = 5;` is used to declare a variable
- `let x: i32 = 5;` is used to declare a variable with type
- `let x = 5;` is used to declare a variable with type inference
- pass variable to function by reference `fn foo(x: &i32) { }`
- pass variable to function by value `fn foo(x: i32) { }`
- function call `foo(5);`
- import library `use std::io;`
- read from console `let mut input = String::new(); io::stdin().read_line(&mut input).unwrap();`




# How to compile 

```bash
rustc main.rs
```

this will create a binary file `main` which you can run using `./main` in Linux `main.exe` in windows