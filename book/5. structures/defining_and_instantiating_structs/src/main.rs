// structs similar to tuples.
// but here you don't need to memorize the order.
// each peace of data can be named, so it's clear what the values mean.
// basically, classes from C# or Java.

// struct's definition:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// we can define struct in main() but it will not be reachable from other functions.

fn main() {
    // to use struct after we've defined it, we create an *instance*
    // of that struct by specifying concrete values for each of the fields.
    // key: value pattern.
    // order doesn't matter.
    // instances just like variables are immutable by default.
    // but we can make them mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    // to reach field of instance we can use dot notation.
    // also we can change it's value.
    user1.email = String::from("anotheremail@example.com");

    // oh my god. look at that.
    // let mut user2 = User {
    //     email: user1.email,
    //     username: String::from("anotherusername"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count,
    // };
    // cool, but... this...
    let mut user2 = User {
        username: String::from("anotherusername"),
        ..user1
    };
    // And yeah. if part above would be uncommented, the whole thing will break.
    // 'cuz we moved user1 to user2, so user1 is not available.
    // we need to give new values for both "String" keys.
    // bool and and u64 have "Copy" trait. "String" doesn't.

    // tuple structs useful if you want to name tuple and make it different from other tuples.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // yeah, they have the same data type in values,
    // but "black" and "origin" have different types.

    // Unit-like structs are useful when you want to implement some trait,
    // but don't have any data to store in the type itself.
    struct AlwaysEqual;
    let subj = AlwaysEqual;

    // at the end: honorable mention.
    // we used owned "String" type, not the "&str" slice.
    // we can store references in struct's data but until Ch. 10 we will use owned types.
}

fn build_user(email: String, username: String) -> User {
    User {
        // ofc, we can define values like this:
        // email: email,
        // username: username,
        // but...
        email,
        username,
        // looks better.
        // this kind of initialization possible if values and keys have the same name.
        active: true,
        sign_in_count: 1,
    }
}