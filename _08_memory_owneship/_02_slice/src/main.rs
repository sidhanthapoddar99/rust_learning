fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
} // This function returns the index of the first space in a string

fn first_word2(s: &String) -> &str {
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

    let mut s2 = String::from("hello world");

    let word2 = first_word2(&s2);

    // s2.clear(); // This will clear the string

    println!("{}", word2);

}