#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of rect1 is {} square pixels.", area(width1, height1));

    let rect2 = (30, 50);

    println!("The area of rect2 is {} square pixels.", tuple_area(rect2));

    let rect3 = Rectangle { width: 30, height: 50 };

    println!("The area of rect3 is {} square pixels.", rectangle_area(&rect3));

    println!("The area of rect3 is {} square pixels.", rect3.area());

    println!("Rect3: {:?}", rect3);

    println!("Rect3: {:#?}", rect3);

    let rect4 = Rectangle { width: 10, height: 40 };
    let rect5 = Rectangle { width: 60, height: 45 };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    let rect6 = Rectangle::square(3);
    println!("The area of rect6 is {} square pixels.", rect6.area());

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}