#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rec = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rec)
    );

    let rect1 = Rectangle { 
        width: 30, 
        height: 50 
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rec(&rect1)
    );

    println!("rect1 is {:?}", rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rec(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}