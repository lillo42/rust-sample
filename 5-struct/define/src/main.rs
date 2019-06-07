struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("anotheremail@example.com");

    println!("{}", user.email);

    let user1 = build_user(
        String::from("someone1@example.com"),
        String::from("someusername1231"),
    );

    println!("{}", user1.email);

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        username: String::from("someusername1232"),
        ..user1
    };

    println!("{}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);

    println!("{} - {} - {}", black.0, black.1, black.2);

    println!("{} - {} - {}", origin.0, origin.1, origin.2);
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

