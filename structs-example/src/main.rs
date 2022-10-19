#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(30, 60)
    );
    let rect1 = Rectangle {
        width: 30,
        height: 90,
    };
    println!("this cool rect {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_str(&rect1)
    );
}

fn area_str(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
