<b> [<<Home](../Readme.md) </b>



- [Operators present in Rust](#operators-present-in-rust)
  - [Exception in comparision](#exception-in-comparision)
  - [Compount Conditions](#compount-conditions)
- [Control Flow](#control-flow)
  - [`if` Expressions](#if-expressions)
  - [`match` Expressions](#match-expressions)
  - [Loops](#loops)


# Operators present in Rust

- **Arithmetic Operators**: `+`, `-`, `*`, `/`, `%`
- **Comparison Operators**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Logical Operators**: `&&`, `||`, `!`
- **Bitwise Operators**: `&`, `|`, `^`, `!`, `<<`, `>>`
- **Assignment Operators**: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`
- **Misc Operators**: `?`, `..`, `..=`, `:`, `::`, `.`
- **Unary Operators**: `+`, `-`, `!`, `*`, `&`, `++`, `--`
- **Ternary Operator**: `condition ? expr1 : expr2`

## Exception in comparision

- Rust does not have a ternary operator. Instead, we can use an `if` expression to achieve the same result.
- Rust does not have an increment or decrement operator. Instead, we can use `+= 1` or `-= 1` to increment or decrement a variable.

unlike other languges the comparison `cannot` be done between `different types of variables`. For example, we cannot compare an integer to a float.

```rust
fn main() {
    let cond = 2.2 <= 3;
    println!("cond: {}", cond);
}
```

result for the above code is 

```
error[E0277]: can't compare `{float}` with `{integer}`
 --> src\main.rs:2:20
  |
2 |     let cond = 2.2 <= 3;
  |                    ^^ no implementation for `{float} < {integer}` and `{float} > {integer}`
  |
  = help: the trait `PartialOrd<{integer}>` is not implemented for `{float}`
  = help: the following other types implement trait `PartialOrd<Rhs>`:
            isize
            i8
            i16
            i32
            i64
            i128
            usize
            u8
          and 6 others

error[E0308]: mismatched types
 --> src\main.rs:2:23
  |
2 |     let cond = 2.2 <= 3;
  |                       ^ expected floating-point number, found integer

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `_06_condition_flow` (bin "_06_condition_flow") due to 2 previous errors
```
insted we can use the below code to compare the float and integer.

```rust
fn main() {
    let cond = 2.2 <= 3.0;
    println!("cond: {}", cond);
}
```

result for the above code is 

```
cond: true
```

## Compount Conditions

Multiple conditions can be combined using logical operators.

- `&&` is the logical `AND` operator. It returns `true` if both conditions are `true`.
- `||` is the logical `OR` operator. It returns `true` if at least one of the conditions is `true`.
- `!` is the logical `NOT` operator. It returns the opposite of the condition.

precedence of the logical operators is `!` > `&&` > `||` except when the paranthesis is used.

```rust
fn main(){
  let number = 6;
  if number % 4 == 0 && number % 3 == 0{
    println!("number is divisible by 4 and 3");
  }
  if number % 4 == 0 || number % 3 == 0{
    println!("number is divisible by 4 or 3");
  }
}
```

# Control Flow

## `if` Expressions

```rust
fn main(){
  let number = 3;
  if number < 5{
    println!("number < 5");
  } 
  else if number == 5{
    println!("number = 5");
  }
  else {
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


## `match` Expressions

`match` is similar to `switch` in other languages. It compares a value to a series of patterns and then executes code based on the pattern that matches the value.
```rust
fn main(){
  let number = 3;
  match number{
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("something
    }
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