
fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");

    let _float64 = 2.0; // f64
    let _float32: f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup1= (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

}
