fn main() {
    // basically, here we take a look at the "if"
    // expression and loops.
    // like... it's control flow...

    let number = 3;

    if number < 5 {
        println!("less than five.");
    } else {
        println!("higher than five.");
    }

    if number != 0 {
        println!("number is not equals to zero.");
    }

    // else if like in other languages.

    let condition = true;
    // values must be the same type.
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // in Rust we have three kinds of loops: loop, while, for.
    // loop() - was before. I can't use long "-" in arch.(((
    // just an infinite loop.
    // uses "break" and "continue".

    let mut counter = 0;
    let result = loop {
        counter += 1;

        // oh... my "if" statement was like if()...
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // 20.

    // basic while loop.
    // basic for loop.
}
