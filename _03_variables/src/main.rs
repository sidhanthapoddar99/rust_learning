fn main() {
    

    println!("------------------Variables---------------------");
    // immutable variable
    let x = 5;

    // x = 6; // error: cannot assign twice to immutable variable `x`

    // mutable variable
    let mut y = 10;
    y = 11; // no error 
    // y = "apple"; // error: expected integer, found `&str`
    // note there will be a warning if the variable is not used 


    // shadowing
    let z = 15;
    let z = "apple";

    // const

    const MAX_POINTS: u32 = 100_000;
    // const MAX_POINTS: u32 = 100_000; // error: cannot assign twice to immutable variable `MAX_POINTS`
    // const MAX_POINTS = 100_000; // error: cannot infer type for `MAX_POINTS`


    // shadowing allows us to change the type of the variable

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("MAX_POINTS: {}", MAX_POINTS);

    println!("------------------Scoping of variables---------------------");

    let x = 5; // x is valid from this point forward
    
    println!("x: {}", x);

    {
        let x = 10; // y is valid from this point forward
        println!("x: {}", x);

    } 

    let x = x + 1;

    println!("x: {}", x);
    
    // output will be 5, 10, 6
    // x assigned to 5
    // x assigned to 10 in interior scope
    // x assigned to x + 1 but for exterior scope x is still 5 as we have redefined x in interior scope

    println!("------------------Scoping of variables 2---------------------");

    let x = 5; // x is valid from this point forward
    
    println!("x: {}", x);

    {
        let x = x + 2; // y is valid from this point forward
        println!("x: {}", x);

    } 

    let x = x + 1;

    println!("x: {}", x);
    
    // output will be 5, 7, 6
    // x assigned to 5
    // x assigned to 7 in interior scope, first x is used from exterior scope and then added 2 to it and assigned to x which is in interior scope so x_interior = x_exterior + 2
    // x assigned to x + 1 but for exterior scope x is still 5 as we have redefined x in interior scope

    println!("------------------Compound types---------------------");

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let arr: [i32; 5] = [0;5];

    let zero_vector: Vec<i32> = vec![0; 5];
    

    
}
