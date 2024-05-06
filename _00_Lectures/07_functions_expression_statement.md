<b> [<<Home](../Readme.md) </b>

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

# Returning Values

Returned values are not named, but their type is defined using an arrow.
```rust
fn five() -> i32{
  5
}
```
This value will return the number 5.

One thing to note is that `rust can return value` but it cannot return satements
```rust
fn main(){
  let x = 5;
  let y = {
    let x = 3;
    x+1
  };
  println!("The value of y is: {}", y);
}
```


# Pass by Reference

- `&` is used to pass by reference.
- `*` is used to dereference a reference.

Example of referencing and dereferencing:

```rust
fn main(){
  let x = 5;
  let y: & i32 = &x; //referencing x
  println!("The value of y is: {}", y);
  let z = *y; //dereferencing y
  println!("The value of z is: {}", z);
}
```



Rust uses pass by reference by default. This means that the value is not copied, but the reference to the value is passed. This is done by using the `&` symbol.
```rust

fn some_function(x: &i32){
  println!("The value of x is: {}", x);
}

fn main(){
  let x = 5;
  some_function(&x); //passing the reference to x
}

```

# Mutable References

To allow a function to modify a value, the reference must be mutable. This is done by using the `&mut` symbol.
```rust

fn some_function(x: &mut i32){
  *x += 1; // dereferencing x and incrementing the value
}

fn main(){
  let mut x = 5;
  some_function(&mut x); //passing the mutable reference to x
  println!("The value of x is: {}", x);
}

```

# Statenments and Expressions

Rust is an expression-based language. Statements only return the unit value, whereas expression return the result of the expression. An example statement is `let x = 10;`. An expression could be `5+6`; this returns the value 11.

Calling a function or macro is an expression. Expressions do not include an ending semicolon,
otherwise they will be treated as statements. An example of this is as following with the line `x+1`.
```rust
fn main(){
  let y = {
    let x = 3;
    x+1
  }; // x+1 is an expression

  println!("The value of y is {}", y);
}
```

Rust returns expressions, but not statements. This is why the following code will not work:


# Other note

We cannot store statements in a variable. We can only store expressions in a variable.
 