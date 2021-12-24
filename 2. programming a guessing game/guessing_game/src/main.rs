use std::io; // providing library for input/output in program's scope.
use rand::Rng; // providing rng library from rand crate.
use std::cmp::Ordering; // bringing a type 'Ordering' into scope from the standard library.

// 'Ordering' is a enum which have variants: 'Less', 'Greater' and 'Equal'.

fn main() {
    println!("Guess the number!"); //println! macro.

    // creating immutable variable 'secret_number' that uses thread_rng from rand crate.
    // generating value from range [1:101).
    // The rand::thread_rng function will give us the particular
    // random number generator that we’re going to use:
    // one that is local to the current thread of execution
    // and seeded by the operating system.
    let secret_number = rand::thread_rng().gen_range(1..101);

    // 'loop' keyword creates an infinite loop.
    loop {
        println!("Please input your guess."); //println! macro.

        // creating a variable 'guess' where we will store user's number.
        // 'let' used to create a variable.
        // in Rust variables are immutable by default. talk 'bout that will be in chapter three.
        // using mut before variable's name let us make it mutable.
        // 'String::new()' is a function that returns a new instance of a String.
        // '::' syntax indicates that 'new' is an assosiated function of the 'String' type.
        let mut guess = String::new();

        // calling assosiated function 'stdin()' from 'io' library we provided in the beginning.
        // this part calls the 'read_line' method on the standart input handle to get input from the user.
        // And we passing one argumetn that is '&mut guess'.
        //*'read_line' will parse whatever user is typed in and !APPEND! it into a string.
        // so this is why we set 'guess' variable mutable.
        // '&' means that it is reference.
        // used to not allocate another memory cluster for variable.

        // it would be difficult to read so we divide it.
        // the 'expect' is used for handling potencial failure.
        // coz read_line also returns value with io::Result type that is enumeration.
        // We can get 'Ok' or 'Err'.
        // expect will either crush program or take the return value that 'Ok' is holding.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // variable shadowing. can be used to change variable type.
        // The colon (:) after guess tells Rust
        // we’ll annotate the variable’s type.

        // let guess: u32 = guess.trim()
        //    .parse()
        //    .expect("Please type a number!");

        // switching from 'expect' to 'match'
        // statement is one way of moving from crashing
        // on an error to handling the error.
        // underscore means 'catchall value'.
        // we actually want to catch all 'Err' values.
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };
        // println! macro have a placeholders!!!
        println!("You guessed: {}", guess);

        // 'cmp' method compares two values and returns a variant of 'Ordering' enum
        // expression 'match' is made up of 'arms'.
        // 'arm' consists of a pattern and the code that
        // should be run if the value given to the beginning
        // of the match expression fits that arm's pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
