fn above_function() {
    println!("Above function.");
}

fn main() {
    above_function();

    println!("Main function!");

    below_function();

    function_with_param(69);

    function_with_params(8, "slices");

    let y = {
        let x = 3;
        x + 1 // at first here was an ';'. just a habit.
    }; // ';' should be here.

    println!("The value of y is: {}", y);

    // expession above is a block that, in tihs case,
    // evaluates to 4.
    // oh... um... okay. this "semicolon by habit"
    // is described in book.

    // in Rust we don't name return values, but we
    // declare their type after an arrow( -> ).
    println!("10 + 5 is equals {}", add_five(10));
}

fn below_function() {
    println!("Below function.");
}

fn function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_params(x: i32, item: &str) {
    println!("Are {} is enough of {}?", x, item);
}


// perfect function in Rust.)0))
// jk. but this is a valid function.
// note that the function's return type is specified too,
// as "-> i32".
// if we will write it like "let x = five();"
// will show that we're using the return value of a
// function to initialize variable.
fn five() -> i32 {
    5
}

// most functions in rust return the value of final
// expression.
fn add_five(x: i32) -> i32 {
    x + 5 // note that we don't have semicolon here.
}