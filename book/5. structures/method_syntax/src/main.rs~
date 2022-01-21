#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // we need to read so use &self.
    // if we want to edit - &mut self.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
        // or just self.width for getter.
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// we can create methods in separated impl blocks.

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// the whole thing above - default class in Java.
// struct - constructor;
// impl - getter(s)/setter(s) and override.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}", rect1.width);
    }

    let sq = Rectangle::square(5);
    println!("Square 5x5 is {:#?}", sq);
}

// methods are similar to the functions.
// but they have "self" as a first parameter.
