<b> [<<Home](../Readme.md) </b>


# Ownership
 Rust has no garbage collection but works on a basis of ownership.

## The Stack and the Heap
The stack is LIFO. Data is `push`ed to the stack and `pop` removes it from the stack.

All data on the stack must have a known, fixed size. Data with an unknown size, or a size that might change must be put on the heap. When data is allocated on the heap, the memory allocator finds a block of data the size that has been requested and returns a pointer to this a pointer to this memory location.

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




## Ownership Rules

- Each value in Rust has a variable called its owner
- There can only be one owner at a time. There can be multiple references to a value but only one owner.
- When the owner goes out of scope, the value will be dropped


## Variable Scope

A scope is the range within a program of which an item is valid.
```rust
#![allow(unused)]
fn main(){
  let s = "hello";
  // s is valid here until }
}
```

The value `s` refers to a string literal. The variable is valid from where it is declared to the end of the current scope.

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
