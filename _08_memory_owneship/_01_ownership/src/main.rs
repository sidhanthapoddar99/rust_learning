fn main() {
    
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("s1: {}", s1); // error: value borrowed here after move
    println!("s2: {}", s2);

}