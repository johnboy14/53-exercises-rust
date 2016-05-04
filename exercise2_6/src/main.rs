use std::io;
extern crate time;

fn read_number(s: &str) -> i32 {
    let mut n = String::new();
    println!("{}", s);
    io::stdin().read_line(&mut n).expect("Error occured");
    match n.to_string().trim().parse::<u32>() {
        Ok(x) => x as i32,
        Err(_) => return read_number("Please enter a positive number")
    }
}

fn main() {
    let age = read_number("What is your current age?");
    let retirement_age = read_number("What age would you like to retire?");
    let years_to_retirement = retirement_age - age;
    if years_to_retirement < 0 {
        println!("Time to retire");
    } else {
        let current_year = 1900 + time::now().tm_year; //why does this return number of years from 1900
        println!("You have {} years left until you retire.  It's {}, so you can retire in {}.",
            years_to_retirement, current_year, current_year + years_to_retirement);
    }
}
