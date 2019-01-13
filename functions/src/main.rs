fn main() {
    another_function(10);
    body();

    let mut x = five();
    println!("The x of number is:{}", x);

    x = plus_one(x);
    println!("The new x of number is:{}", x);
}

fn another_function(number: i32) {
    println!("The value of number is:{}", number);
}

fn body() {
    // comments
    let y = {
        let x = 3;

        x + 1
    };

    println!("The value of y is: {}", y);
}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}