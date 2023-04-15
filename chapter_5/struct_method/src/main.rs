#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    // more parameters method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle{
        length: 50,
        width:30
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // ---

    let rect_1 = Rectangle { length: 50, width:50};
    let rect_2 = Rectangle { length:40, width:10};
    let rect_3 = Rectangle { length:45, width:60};

    println!("Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect_1 hold rect_3? {}", rect_1.can_hold(&rect_3));

    // ---

    let sq = Rectangle::square(3);

    println!("sq is {:#?}", sq);
}