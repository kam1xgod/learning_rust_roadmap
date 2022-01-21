#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// I will not write comments from now.
// Only if it's necessary.

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "(Simple) The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "(Struct) The area of the rectangle is {} square pixels.",
        area_by_struct(&rectangle)
    );

    println!("rect is {:?}", rectangle);
    println!("rect is {:#?}", rectangle);

    // dbg!() macro takes ownership, prints info in stderr style and returns ownership.
    let scale = 2;
    let rectlangle2 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    dbg!(rectlangle2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 //destruct tuple.
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
