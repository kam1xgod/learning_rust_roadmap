// heap can store data when it's size is unknown
// at runtime.

fn main() {

    the_string_type();
    interacting_clone();
    interacting_move();
    stack_only_copy();
    ownership_and_functions();
    return_values_and_scope();
    returning_ownership_of_params();

    // "s" is not valid here.
    let _s = "hello"; // "s" is valid from this line.

    // the main thing is that literals (this "s")
    // can't be mutated.

    // "s" is still be valid.
} // "s" is no longer valid 'cus scope is now over.

// the "String" type is more complex than ones that was
// covered previously.
fn the_string_type() {
    // the "s" above was hardcoded and used stack.
    // in "s" that is "String" type we use heap.
    // because there can be situations where we want,
    // for example, store value from user's input.
    let _s = String::from("Hello.");

    // this "s" can be mutated.
    let mut s = String::from("Hello.");
    s.push_str(" Bye?"); // append literal to String.
    println!("{}", s);

    // difference is how these two types deal with memory.
    // literal will be faster.
    // with String we need to request amount of memory
    // at runtime; and then return memory to the allocator
    // when we're done with our String.

    // this second part is different in Rust because
    // it doen't gave garbage collector.
    // so memory is automatically returns when
    // variable is out of scope.
    // Rust calls "drop" function at the closing
    // curly bracket.
}

fn interacting_move() {
    let x = 5;
    let _y = x;
    // this two "5" values are pushed onto stack.

    // now this one.
    let s1 = String::from("hello");
    let _s2 = s1;
    // yeah, it will do the same thing as code above.
    // ! but!
    // String is made up of three parts:
    // pointer to memory (refers to a heap);
    // length (how much memory in bytes the content of String currently using.);
    // capacity (amount of memory that String received from allocator).
    // the two last parts are different but we ignore it for now.
    // when we copy s1 into s2 we copy all the parts,
    // but we do not copy the data on the heap.

    // in this example, as we know, s1 and s2 will try to
    // free same memory. This is double free error.
    // to ensure this Rust considers "s1" to no longer be valid.
    // Rust won't free space when s1 goes out of scope.

    // * so "s1" was *moved* to "s2".
}

fn interacting_clone() {
    // if we want to copy heap data, not just the stack
    // we use method "clone()".
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    // here we also copy pointers array.
}

fn stack_only_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // because integers have a known size.
    // so, difference beetween deep and shallow copying
    // here, so calling "clone" wouldn't do anything
    // different from the usual shallow copying and we
    // can leave it out.

    // if a type implements the "Copy" trait, an
    // older variable is still usable after assignment.

    // basically, all simple scalar values can implement
    // "Copy", and nothing that requires allocation or
    // is some form of resource can implement "Copy".
    // for example, integers, boolean, floats, char.
    // and also tuples which contains them.
}

fn ownership_and_functions() {
    let s = String::from("hello."); // "s" comes into scope.

    takes_ownership(s); // "s" value moves into the func.
                        // and so is no longer valid here.
    let x = 5;          // x comes into scope.

    makes_copy(x);      // x would move into the func.
                        // but i32 is Copy, so it's okay
                        // we still can use x afterward.
} // Here, x goes out of scope, then s. But because s value was
// moved nothing special happens.

// some_string comes into scope.
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called.
// The backing memory is freed.

// some_integer comes into scope.
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special.

fn return_values_and_scope() {
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its
                                 // return value into the function
                                 // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                  // some_string is returned and
                                 // moves out to the calling
                                 // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string // a_string is returned and moves out to the calling function
}

// when a variable that includes data on the heap goes
// out of scope, the value will be cleaned up by "drop"
// unless the data has been moved to be owned by another
// variable.

// it's possible to return multiple values using tuple.
// looks like we've returned ownership back.
// but better use references...
fn returning_ownership_of_params() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}