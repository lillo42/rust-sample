#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    rect1();

    let rec = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rec(&rec)
    );

    println!("rect1 is {:?}", rec);
}

fn rect1() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );   
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_rec(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}