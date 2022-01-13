fn main() {
    // same thing as in previous part but with reference.
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    change_correct();
    ok_mixing();
    dangling_refs_and_pointers();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // scope doesn't have ownership of 's' so
// nothing happens.

// this function won't work because
// it uses the reference that is immutable.
// fn change_error() {
//     let s = String::from("Hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world.");
// }

// this function with little fixes will work.
fn change_correct() {
    let mut s = String::from("Hello");

    change(&mut s);
}

// as you can see, we use "&mut String" instead of
// simple "&String".
fn change(some_string: &mut String) {
    some_string.push_str(", world.");
} // but we can use only one mutable
// reference to one piece of data.
// this code will give error:
//      let mut s = String::from("hello");
//      let r1 = &mut s;
//      let r2 = &mut s;
//      println!("{}, {}", r1, r2);
// we can't borrow 's' as  mutable more than once
// at a time.
// we can use curly brackets to create new scope
// and use multiple mutable references.

// similar situation with combining mutable and immutable:
//      let mut s = String::from("Hello");
//      let r1 = &s; // no problems here.
//      let r2 = &s; // no problems here too.
//      let r3 = &mut s; // here is the THICK PROBLEM.

// multiple immutable refs are okay because they are not
// able to change data, only read it.

// but everything is okay if we do something similar:
fn ok_mixing() {
    let mut s = String::from("Hello.");

    let r1 = &s; // okay.
    let r2 = &s; // okay.
    println!("{} and {}", r1, r2); // end of scopes.
    // "r1" and "r2" will not be used after this.
    // its called Non-Lexical Lifetimes (NLL).

    // "r3" and "r1" with "r2" scopes don't overlap.
    let r3 = &mut s; // okay.
    println!("{}", r3);
}

// dangling references and pointers.
// it basically means that pointer refers to a location
// in memory that may have been given to someone else.

fn dangling_refs_and_pointers() {
    let _reference_to_nothing = no_dangle();
}

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// but we can return value directly.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// basic rules:

//      At any given time, you can have either
//  one mutable reference or any number of
//  immutable references.

//      References must always be valid.