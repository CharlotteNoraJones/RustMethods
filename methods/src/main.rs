fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    let rect3: Rectangle = Rectangle {
        width: 40,
        height: 60,
    };

    let sq: Rectangle = Rectangle::square(3);

    println!("The are of the rectange is {} square pixels.", rect1.area());

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}

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

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
