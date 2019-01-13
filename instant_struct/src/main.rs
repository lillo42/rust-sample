fn main() {
    println!("Hello, world!");

    let user1 = build_user("someusername123", "someone@example.com");

    println!("{}", user1.email);

    let mut user2 = build_user("someusername123", "someone@example.com");
    user2.active = false;

    println!("{}", user2.active);

    let user3 = User {
        email: String::from("another@example.com"),
        user_name: String::from("anotherusername567"),
        ..user1
    };

    println!("{}",user3.user_name);
    println!("{}",user3.active);
    println!("{}",user3.sign_in_count);

    let point = Point(10,20,30);
}


fn build_user(user_name: &str, email: &str) -> User {
    User {
        email: email.to_owned(),
        user_name: user_name.to_owned(),
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);