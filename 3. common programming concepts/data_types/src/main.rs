fn main() {

    // this is bad:
    // let guess = "235".parse().expect("Not a number.");
    // this is good:
    let guess: u32 = "235".parse().expect("Not a number.");
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
    
}
