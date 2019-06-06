fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    change(&mut s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String {
//    let s = String::from("dangle");
//    &s
//}

fn no_danlge() -> String {
    let s = String::from("dangle");
    s
}