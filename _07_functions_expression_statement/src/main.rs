
fn check_pass_by_reference(x: &i32) {
    println!("x inside the function: {}", x);
}

fn pass_mutable (y: &mut i32) {
    println!("y inside the function before change: {}", y);
    *y = 20;
    println!("y inside the function after 1st change: {}", y);

    // copy the value of y to a new variable but it can be used without dereferencing


    //y = 30; // error: expected `&mut i32`, found integer
    // why? because y is a reference to i32, so we need to dereference it first
    // why it prints 20 without dereferencing? because println! macro does it for us
    println!("y inside the function after change: {}", y);
}

fn returning_sum_of_array (arr: &[i32]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }
    sum
}

fn using_return_Statement() -> i32 {
    let x = 10;
    let y = 20;

    let z = {
        let x = 5;
        x + y
    };

    println!("z inside the function: {}", z);

    return z;
}

fn main() {

    let x: i32 = 10;
    println!("x before the function: {}", x);
    check_pass_by_reference(&x);
    println!("x after the function: {}", x);


    // referenceing and dereferencing
    let a = 10_i32; // 10 as i32, 
    let b : &i32 = &a;
    let c : i32 = *b;

    println!("\n");

    let mut y: i32 = 10;
    println!("y before the function: {}", y);
    pass_mutable(&mut y);
    println!("y after the function: {}", y);


    println!("\n");

    let x = 10;

    let y = {
        let x = x + 5;
        x + 1
    };

    
    println!("y is a block expression: {}", y);
    println!("x: {}, y: {}", x, y);

    println!("\n");

    let arr = [1, 2, 3, 4, 5];
    let sum = returning_sum_of_array(&arr);
    println!("sum of the array: {}", sum);

    println!("\n");

    let z = using_return_Statement();
    println!("z: {}", z);
}
