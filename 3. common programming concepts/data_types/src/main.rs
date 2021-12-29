fn main() {

    // this is bad:
    // let guess = "235".parse().expect("Not a number.");
    // this is good:
    let _guess: u32 = "235".parse().expect("Not a number.");
    // ": u32" is calles annotation. it infer compiler
    // what actual type we wanna use after parsing.

    // in Rust we have four scalar types:
    // integers, float-pointing numbers,
    // booleans and characters.

    // Integer:

    // we already used u32 in code.
    // it refers to unsigned 32-bit integer value.
    // Length	Signed	Unsigned
    // 8-bit	i8  	u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // i8 can store numbers from -128 to 127 (2^7; 2^7-1);
    // u8 can store numbers from 0 to 255 (2^8 - 1);

    // Floating-point:
    // f32 and f64 for 32-bit and 64-bit.

    let _x = 2.0; // f64 by default.

    let _y: f32 = 3.0; // f32

    // Numeric Opertations.

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    // Boolean. srsly?

    //Char.

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Actually in official rust book here's some UTF-8 values.
    // In VS Code I've got a warning about second and third syms.
    // to divide char and string rust uses single and double quotes.
    // test:
    // println!("{}", heart_eyed_cat);
    // it simply prints empty box.
    // may be I just don't have a font for emojis.

    // Compound types.

    // They can group multiple values into one type.
    // Tuple.
    // Tuples have fixed length. To define it we use this:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tuple is a single element.
    // To get data from it use something like this:
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    // This is called destructuring.

    // Also we can simply use indexes:
    let _x = tup.0;
    let _z = tup.2;

    // Tuple without any values, (), is a special type that has only one value,
    // also written (). The type is called the unit type and the value is called the unut
    // value. Expressions implicitly return the unit value if they don't retunrn any
    // other value.

    // Array type.
    // Bassically, the arrays...
    let _arr = [1, 2, 3, 4, 5, 69];

    // uses stack insead of heap. (see Ch. 4)
    // arrays isn't flexible as the vector.
    // most of the time you use vector if you unsure what to choose: array of vector.
    // vectors discussed in more details in ch. 8.

    // to specify type of each elements and length of array.
    let _arr: [i32; 5] = [1, 2, 3, 4, 5]; // 32-bit int with 5 elements.

    // to specify value of each element and length of array.
    let _arr = [3; 5]; // [3, 3, 3, 3, 3]

    // in Rust accessing the array's elements is default, like in any other languages.
    let _first = _arr[0];

    // Rust will panic if index is out of bounds. see ch. 9 for Rust's erorr handling.
}
