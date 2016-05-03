use std::io;

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
    let x = read_number("What is the first number?");
    let y = read_number("What is the second number?");

    let add = |n: i32, t: i32| -> i32 {n + t};
    let sub = |n: i32, t: i32| -> i32 {n - t};
    let mult = |n: i32, t: i32| -> i32 {n * t};
    let divis = |n: i32, t: i32| -> i32 {n / t};

    println!("{} + {} = {}", x, y, add(x, y));
    println!("{} - {} = {}", x, y, sub(x, y));
    println!("{} * {} = {}", x, y, mult(x, y));
    println!("{} / {} = {}", x, y, divis(x, y));
}
