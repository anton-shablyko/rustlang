#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Area of rectangle is {} pxs", area_basic(width1, height1));

    // TUPLES
    let rect1 = (30, 50);
    println!("Area of rectangle is {} pxs", area_tuples(rect1));

    // STRUCT
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:#?}", rect);
    println!("Area of rectangle is {} pxs", area_struct(&rect));
}

fn area_basic(w: u32, h: u32) -> u32 {
    w * h
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
