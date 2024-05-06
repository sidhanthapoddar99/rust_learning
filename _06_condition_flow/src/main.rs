fn main() {
    // let cond = 2.2 <= 3; // error: no implementation for `{float} <= {integer}`
    let cond = 2.2 <= 3.0;
    println!("cond: {}", cond);
}
