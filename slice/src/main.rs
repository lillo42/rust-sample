fn main() {
    let mut s = String::from("hello world");
    
    let word = first_word(&s);

    println!("the position of first word: {}", word);

    s = String::from("hello world");
    let hello = &s[0..=4];
    let word = &s[5..=10];

    println!("the first word: {}", hello);
    println!("the second word: {}", word);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}