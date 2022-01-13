use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1, // if n = 0 - return 1;
        1 => 1, // if n = 1 - return 1;
        // let n = 10, function will go down to 0
        // summarizing all the numbers returned by
        // fibonacci n - 1 and n - 2 for all numbers.
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci generator");
    println!("Type \"quit\" to end the program");

    loop {
        // creating "n" variable which is empty string.
        let mut n = String::new();

        println!("\nEnter a positive integer:");

        // reading the "n".
        io::stdin().read_line(&mut n).expect("Failed to read line");

        // quit from program.
        if n.trim() == "quit" {
            break;
        }

        // default match pattern.
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // printing results.
        println!("{}", fibonacci(n));
    }
}