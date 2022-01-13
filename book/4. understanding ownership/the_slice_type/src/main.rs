fn main() {
    // small problem:
    // write a function that takes a string and
    // returns the first word it finds in that string.
    // if there's no spaces in the string, the whole
    // string must be one word, so the entire string
    // should be returned.

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the '5'.

    let word = first_word_slices(&s);

    println!("the first word is: {}", word);

    s.clear(); // clear string, make it equal to "".

    // this will give compile-time error
    // because of immutable ref after mutable ref:
    // println!("the first word is: {}", word);

    // "word" still have '5' but there is no more string
    // that we could meaningfully use this value.
    // "word" is now totally invalid.

    // we still can run the program without any errors.
    // but if we try to gen this first word by index
    // in "word" variable we will meet the bug.
    // we get out of sync with the data in 's'.
    // so the time for slice has come.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to bytes array.

    // iterator over the array of bytes using "iter" method.
    // we will talk 'bout "iter" later.
    // for now, know that "iter" is a method that
    // returns each element in a collection and that
    // "enumerate" wraps the result of "iter" and
    // returns each element as part of tuple instead.
    // the first elem is the index, the second - ref to elem.
    for (i, &item) in bytes.iter().enumerate() {
        // in "for" loop we specify a pattern to destructure
        // that tuple: 'i' - index, "&item" - single byte.
        // we use '&' because we get ref from "iter().enumerate()".
        // "b' '" is the byte literal syntax to find
        // byte that represents space.
        if item == b' ' {
            // we return the position if we find something.
            return i;
        }
    }

    // or we return length of the entire string.
    s.len()
}

// now we can find the index of the end of the first word.
// but the "usize" itself means nothing for "&String".
// because of that there is no guarantee that it will
// still be valid in the future.

// string slices.
// string slice is a reference to the whole String
// but with extra [x..y] bit.
// so it's the reference to a portion of the String.
// x - starting index; y - ending index.
// example:
// let hello = &s[0..5];
// let world = &s[6..11];
// this is the same like this:
// let hello = &s[..5];
// let world = &s[6..];
// let entire_word = &s[..];

// Note: String slice range indices must occur
// at valid UTF-8 character boundaries.
// If you attempt to create a string slice
// in the middle of a multibyte character,
// your program will exit with an error.

// it's like a function above but with slices.
// string slice is written as &str.
fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// we now have a straightforward API that's much
// harder to mess up.

// String literals are slices.
// let s = "Hello, world!";
// the type of 's' here is "&str":
// it's a slice pointing to that specific
// point of the binary.
// this is also why string literals are immutable;
// &str is an immutable ref.

// String slices as Parameters.
// fn first_word(s: &str) -> &str {}
// allows us to use the same function on both:
// &String values and &str values.

// Other slices.
// String slices specific to strings, yeah.
// let a = [1, 2, 3, 4, 5];
// let slice = &a[1..3];
// assert_eq!(slice, &[2, 3]);
// this slice has the type &[i32].
// it stores a ref to the first element and a length.
// this kind of slice used for all sorts of collections.