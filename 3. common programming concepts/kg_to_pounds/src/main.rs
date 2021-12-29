use std::io;

fn main() {
    // i want to wrap it in loop.
    //loop {
        println!("kg: ");
        let mut kg = String::new();
        io::stdin()
            .read_line(&mut kg)
            .expect("Failed to read line.");

        let kg: f32 = match kg.trim().parse() {
            Ok(kg) => kg,
            Err(_) => 0.0,
        };

        println!("{} in lbs equals to: {:.4}", kg, convert(kg));

    //}
}

fn convert(kg: f32) -> f32 {
    kg * 2.2
}