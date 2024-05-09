<b> [<<Home](../Readme.md) </b>

- [Ownership](#ownership)
  - [The Stack and the Heap](#the-stack-and-the-heap)
    - [Example of stack ](#example-of-stack-)
    - [Example of heap ](#example-of-heap-)
  - [Memory \& Allocation](#memory--allocation)
    - [Data Interaction: Move](#data-interaction-move)
    - [Data Interaction: Clone](#data-interaction-clone)
    - [Data Interaction: Copy](#data-interaction-copy)
  - [Ownership Rules](#ownership-rules)
- [Variable Scope](#variable-scope)
  - [Ownership \& Functions](#ownership--functions)
  - [Return Values \& Scope](#return-values--scope)
- [References \& Borrowing](#references--borrowing)
  - [Mutable References](#mutable-references)
  - [Dangling References](#dangling-references)
  - [Multiple References](#multiple-references)
- [Slice type](#slice-type)
  - [String Slice](#string-slice)
  - [Other Slices](#other-slices)
  - [The `string` Type](#the-string-type)



# Ownership
 Rust has no garbage collection but works on a basis of ownership.


| Method | Pros | Cons | Example |
|---|---|---|---|
Garbage Collection | Error Free <br> Faster write time  | No Control over memory <br> Slower and unpredictable runtime preformance <br> Large Program size | Java <br> Python
Manual Memory <br> Management | Full control over memory <br> Faster runtime performance <br> smaller program size| Error prone <br> Difficult to maintain <br> Difficult to scale <br> slower write time | C <br> C++
Ownership | Error free <br> Faster runtime performance <br> Control over memory <br> Smaller program size | Learning curve <br> Slower write time | Rust


## The Stack and the Heap
The stack is LIFO. Data is `push`ed to the stack and `pop` removes it from the stack.

- All data on the `stack` must have a known, `fixed size`. 
- Data with an unknown size, or a size that might change must be put on the heap. 
- When data is allocated on the heap, the memory allocator finds a block of data the size that has been requested and returns a pointer to this a pointer to this memory location.

  - Pushing to the stack is much faster than allocating to the heap.
  - Accessing data in the heap is much slower than accessing data on the stack.

When calling a function, values passed into the function and the function's local variables get pushed onto the stack. When the function is over, those data get popped off the stack. This is of course an oversimplification as this is optimised using registers and other methods within the compiler; but this is out of the scope of this sheet. 



<table>
    <tr>
        <th>Stack</th>
        <th>Heap</th>
    </tr>
    <tr>
        <td>Fast</td>
        <td>Slower</td>
    </tr>
    <tr>
        <td>Fixed size</td>
        <td>Unknown size</td>
    </tr>
    <tr>
        <td>LIFO</td>
        <td>Random access</td>
    </tr>
    <tr>
        <td>Push and pop</td>
        <td>Alloc and dealloc</td>
    </tr>
</table>



### <u>Example of stack </u>

```rust
fn example(){
    let x = 5;
    let y = 6;
}

fn main(){
    let a = 2;
    let b = a;
    example();
}
```

step by step stack allocation


<table>
    <tr>
        <th>Address</th>
        <th>Step1</th>
        <th>Step2</th>
        <th>Step3</th>
        <th>Step4</th>
        <th>Step5</th>
    <tr>
    <tr>
        <td>1</td>
        <td> - </td>
        <td>a = 2</td>
        <td>a = 2</td>
        <td>a = 2</td>
        <td> - </td>
    </tr>
    <tr>
        <td>2</td>
        <td> - </td>
        <td>b = 2</td>
        <td>b = 2</td>
        <td>b = 2</td>
        <td> - </td>
    </tr>
    <tr>
        <td>3</td>
        <td> - </td>
        <td> - </td>
        <td>x = 5</td>
        <td> - </td>
        <td> - </td>
    </tr>
    <tr>
        <td>4</td>
        <td> - </td>
        <td> - </td>
        <td>y = 6</td>
        <td> - </td>
        <td> - </td>
<table>

Explanation:
- step 1 `a` is pushed onto the stack and `a` is copied to `b` and `b` is pushed onto the stack
- step 2 the function `example` is called and `x`, `y` is pushed onto the stack
- step 3 `x`, `y` is popped off the stack
- step 4 `b`, `a` is popped off the stack


### <u>Example of heap </u>

```rust
fn example(){
    let x = String::from("hello");
    let y = String::from("world");
}

fn main(){
    let a = String::from("hello");
    let b = a;
    example();
}
```
- in string instead of stroing data inside stack we store the `reference` of the data in stack and the `data` is stored in heap.
- the `stack reference` is pointing to the `heap data`.



## Memory & Allocation

As the size of string literals is known at compile time the memory can be allocated and the data can be hard coded into the executable.

As the size of a mutable string is unknown at compile time, memory must be allocated onto the heap. Therefore
- The memory must be requested from the OS at runtime
- The memory must be returned to the OS when the program is finished using it

The first part is done automatically when the string is created. This is common across many programming languages.

The second is more difficult. In languages with a garbage collector this is done automatically. However Rust has no garbage collector. In languages without a garbage collector this is done manually and for every allocation a `free` is required to prevent excess memory use or premature memory de-allocation. Rust automatically de-allocates memory when it goes out of scope. In Rust, the command for memory de-allocation is `drop` and this is often done automatically at the end of a scope.
 
### Data Interaction: Move
Multiple variables can interact with the same data.

For primitive data types, if a variables is assigned to another primitive then the value is simply copied to the second one. This is because of the `Copy` trait.
```rust 
let x = 5;
let y = x;
```
However, for non-primitive types (for example, strings), this is not the case. 
```rust
let s1 = String::from("hello");
let s2 = s1;
```
When we assign `s1` to `s2` the string data on the stack (`ptr`, `len`, `capacity`) are copied to `s2` but the data on the heap is not copied. This makes it more efficient as the whole character array is not recreated. 

This can cause a problem if both strings go out of scope at once. Both `s1` and `s2` will try to free the same memory on the heap (using `drop`); this is known as a **double free** error. This is a memory safety bug and can lead to memory corruption and/or security vulnerabilities.

Rust avoids this issue by classing `s1` to be no longer valid; therefore it doesn't need to be freed once it is out of scope. 

For example, if you try to use `s1` after `s2` has be initialised, it will thrown an error.
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
```rustc
error[E0382]: use of moved value: `s1`
--> src/main.rs:5:28
  | 
3 | let s2 = s1;
  |     -- value moved here
4 | 
5 | println!("{}, world!", s1);
  |                        ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait 
```

This is effectively a shallow copy, but because `s1` is invalidated, it is called a **move**

### Data Interaction: Clone
If we do want to create a deep copy of an object, we can use the clone method. 
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This means the heap is being copied and so is less efficient an more expensive.

### Data Interaction: Copy
The `Copy` trait can be added to types which are stored entirely on the stack (i.e. fixed size). `Copy` cannot be implemented if the type or any of its types implement `Drop`.


## Ownership Rules

- Each value in Rust has a variable called its owner
- There can only be one owner at a time. There can be multiple references to a value but only one owner.
- When the owner goes out of scope, the value will be dropped


# Variable Scope

A scope is the range within a program of which an item is valid.
```rust
#![allow(unused)]
fn main(){
  let s = "hello";
  // s is valid here until }
}
```

The value `s` refers to a string literal. The variable is valid from where it is declared to the end of the current scope.


## Ownership & Functions
The semantics of passing a value to a function are similar to those of assigning a value to a variable. Passing a variable to a function will move or copy it, the same as assignment.
```rust
fn main(){
  let s = String::from("hello");  // s comes into scope 

  take_ownership(s);  // s moves into the function and is no longer valid in this scope

  let x = 5;  // x comes into scope

  makes_copy(x);  // x will be copied into the function as it is an i32 and so it 
                  // can still be used in this scope
} // x goes out of scope

fn take_ownership(some_string: String){ // some_string comes into scope
  println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. This frees the memory of some_string
fn makes_copy(some_int: i32){ // some_int comes into scope
  println!("{}", some_int);
} // some_int goes out of scope
```

## Return Values & Scope
Returning values can also transfer ownership.
```rust 
fn main(){
  let s1 = gives_ownership(); // The return value is moved into `s1`

  let s2 = String::from("hello"); // `s2` comes into scope

  let s3 = takes_and_gives_back_ownership(s2);  // `s2` is moved into the method and then the return value is moved into `s3`
}

fn gives_ownership() -> String { // Moves its return value into the calling method
  let s  = String::from("hello");
  s
}

fn takes_and_gives_back_ownership(s: String) -> String { // `s` comes into scope
  s // `s` is returned to the calling function
}
```

Taking ownership of a variable follows the same pattern every time. 

Taking and returning ownership of a variable with every function can be quite tedious so references can be used to prevent this.


# References & Borrowing

References allow you to refer to a value without taking ownership of it. This is known as borrowing.

> Once a variable is passed to a function, it is no longer valid in the calling function. This is known as `move`. However, if you want to use the variable in the calling function after it has been passed to another function, you can use `references`.

 <u>**Example of a move function:**</u>

```rust

fn main(){
    let s1 = String::from("hello");
  
    let len = calculate_length(s1);
  
    println!("The length of '{}' is {}", s1, len);
  }
  
  fn calculate_length(s: String) -> usize
  {
    let length = s.len(); // len() returns the length of a string
    length
  }
```
This will throw an error as `s1` has been moved to `s` and is no longer valid in the calling function.

```bash

❯ cargo run
   Compiling _01_ownership v0.1.0 (F:\Projects\Learning\rust\_08_memory_owneship\_01_ownership)
error[E0382]: borrow of moved value: `s1`
 --> src\main.rs:6:42
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |
4 |     let len = calculate_length(s1);
  |                                -- value moved here
5 |
6 |     println!("The length of '{}' is {}", s1, len);
  |                                          ^^ value borrowed here after move
  |
note: consider changing this parameter type in function `calculate_length` to borrow instead if owning the value isn't necessary
 --> src\main.rs:9:26
  |
9 |   fn calculate_length(s: String) -> usize
  |      ----------------    ^^^^^^ this parameter takes ownership of the value
  |      |
  |      in this function
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let len = calculate_length(s1.clone());
  |                                  ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `_01_ownership` (bin "_01_ownership") due to 1 previous error

```

<u>**Example of a borrow function:**</u>

```rust
fn main(){
    let s1 = String::from("hello");
  
    let len = calculate_length(&s1);
  
    println!("The length of '{}' is {}", s1, len);
  }
  
  fn calculate_length(s: &String) -> usize
  {
    let length = s.len(); // len() returns the length of a string
    length
  }
```
```bash
❯ cargo run
   Compiling _01_ownership v0.1.0 (F:\Projects\Learning\rust\_08_memory_owneship\_01_ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target\debug\_01_ownership.exe`
The length of 'hello' is 5
```

This will work as `s1` is borrowed by `s` and is still valid in the calling function.

## Mutable References
You can also create mutable references by using `&mut` instead of `&`. This allows you to change the value of the reference. By default, references are immutable.

```rust
fn main(){
    let mut s = String::from("hello");
  
    change(&mut s);
  
    println!("{}", s);
  }
  
  fn change(s: &mut String){
    s.push_str(", world");
  }
```

## Dangling References
Rust will not allow you to create dangling references. This is a reference that refers to a location in memory that may have been deallocated. This is a common bug in other languages.

```rust
fn main(){
    let reference_to_nothing = dangle();
  }
  
  fn dangle() -> &String {
    let s = String::from("hello");
    &s
  }
```
This will throw an error as `s` goes out of scope and is deallocated as soon as function ends. Rust will not allow you to create a reference to a deallocated memory location.

```bash
❯ cargo run
   Compiling _01_ownership v0.1.0 (F:\Projects\Learning\rust\_08_memory_owneship\_01_ownership)
error[E0106]: missing lifetime specifier

  --> src\main.rs:5:16
    |
5   |   fn dangle() -> &String {
    |                ^ expected lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    = help: consider giving it a 'static lifetime
    = note: the lifetime must be valid for the lifetime 'static as defined on the function body at 4:1...
```

## Multiple References
You can have multiple references to a value but only one mutable reference. This is to prevent data races. 

```rust

fn case1(){
    let mut s = String::from("hello");
  
    let r1 = &mut s;
    let r2 = &mut s;
    let r3 = &mut s;
  
    println!("{}, {}, and {}", r1, r2, r3);
  }

fn case2(){
    let mut s = String::from("hello");
  
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
  
    println!("{}, {}, and {}", r1, r2, r3);
  }

fn case3(){
    let mut s = String::from("hello");
  
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
  
    println!("{}, {}, and {}", r1, r2, r3);
  }

fn case4(){
    let mut s = String::from("hello");
  
    let r1 = &s;
    let r2 = &s;
  
    println!("{}, and {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);
  
  }


fn main(){
    
    case1();
    case2();
    case3();
  }

```

<u>**Explanation:**</u>

> If you have a mutable reference, you cannot have any other references to the same value.

- case1: This will throw an `error` as you cannot have `multiple mutable references` to a value. This is to prevent
- case2: This will work as you can have `multiple immutable references` to a value.
- case3: This will throw an `error` as you cannot have multiple `mutable references` and `immutable references` to a value at the same time.
- case4: This will not throw an error as the `mutable reference` is created after the `immutable references` scope ends. This is because the `immutable references` are no longer valid when the `mutable reference` is created.


# Slice type

A `slice` is another data type without ownership. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 

## String Slice

A string slice is simply a reference to a substring.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

> *Note: the lower bound is inclusive and the upper bound is exclusive*

This is stored internally as the `starting index` and then the `length of the slice`. 

String literals are stored as slices. They are immutable because the pointer to the beginning of the slice is immutable. 


<u>**String Slices and Functions**</u>

> lets say we want to find the first word in a string.

```rust

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
} // This function returns the index of the first space in a string

fn main(){
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // This will clear the string

    println!("{}", word);

}
```

<u>**Explanation:**</u>

- This will not throw an error as the reference to the string is still valid. However, this will return the index of the first space in the string. 
- If the string is cleared, the reference will `still be valid` but the `data will be different`.
- This causes a problem as the reference is no longer valid. This is known as a `dangling reference` and is a common bug in other languages.

To fix this, we can use string slices.

```rust

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
} // This function returns the first word in a string

fn main(){
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // This will clear the string

    println!("{}", word);

}

```

> Note : string have type `String` and string slices have type `&str`

<u>**Explanation:**</u>
- This will throw an error as the reference is no longer valid. when the `string is cleared`.
- Thus, the function will return the first word in the string as a string slice. 

## Other Slices

Other slices can be created from other contiguous collections (for example vectors and arrays) using the same syntax.



## The `string` Type

To create a mutable string requires the following declaration.
```rust
fn main(){
  let mut s = String::from("hello");
  s.push_str(", world!"); // push_str() appends to the string
  println("{}", s);
}
```
Each string contains 3 pieces of data: a pointer to the character array, the length and the capacity.

For `let s1 = String::from("hello");`, the data would be:
| Name | Value |
|---|---|
| ptr | 0x... |
| len | 5 |
| capacity | 5 |

This data is stored on the stack.

The character array at `0x...` would appear as:
| Index | Value |
|---|---|
| 0 | h |
| 1 | e |
| 2 | l |
| 3 | l |
| 4 | o |

This is stored on the heap.
