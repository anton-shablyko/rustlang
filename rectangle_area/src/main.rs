#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

mod shapes;

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    //BASIC
    let width1 = 30;
    let height1 = 50;
    println!("{}", shapes::rectangle_basic::area(width1, height1));

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

    // Method
    let m_rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of rectangle is {} pxs", m_rect.area());
    println!("{}", m_rect.is_width());
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
