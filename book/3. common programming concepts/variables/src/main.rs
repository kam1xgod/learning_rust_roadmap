fn main() {
    // immutable variables can be useful in some cases.
    // but actually now I can't just think of them.
    // in book there are small data structers.
    // like you can create a new instances and write in more functional way.
    // you will decrease performance but increase clarity.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants can be declared by "const".
    // it's important to use this certain case.
    // all words are in upper case and underscores between.
    // it's also important that immutable and constants are not the same things.
    // constants just can be turned into mutable state.
    const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;

    // shadowing is already known.
    // we used "let x = 5" and "x = 6" before.
    // so now in 'x' variable we store '6'.
    // BUT. we can create new variable with the same name.
    let x = x + 1; // x = 7;

    // check this out.
    {
        let x = x * 2; // x = 14;
        println!("The value of x in inner scope is: {}", x); // prints 14;
    }

    println!("The value of x is: {}", x); // prints 7;

    // SHADOWING IS NOT THE SAME AS "LETTING NEW VARIABLE.
    // we can shadow immutables.
    // we can change type of variable remaining the same name.
    // for example:
    let spaces = "    ";
    let spaces = spaces.len();
    // it was &str but now it's usize.
    // simple "spaces = spaces.len()" will give us a compile error.
}
