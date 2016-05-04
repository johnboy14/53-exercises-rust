use std::io;

fn read_number(s: &str) -> f32 {
    let mut n = String::new();
    println!("{}", s);
    io::stdin().read_line(&mut n).expect("Error occured");
    match n.to_string().trim().parse::<f32>() {
        Ok(x) => x as f32,
        Err(_) => return read_number("Please enter a number")
    }
}

fn main() {
    let units = read_number("Choose your units, (1) for Feet or (2) for Metres");
    let length = read_number("What is the length of the room?");
    let width = read_number("What is the width of the room?");
    let square_feet = length * width;
    if units == 1. {
        println!("You entered dimensions of {} feet by {} feet.", length, width);
        println!("The area is\n{} square feet", square_feet);
    } else {
        println!("You entered dimensions of {} metres by {} metres.", length, width);
        println!("The area is\n{} square metres", square_feet);
    }
}
