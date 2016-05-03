
use std::io;

fn main() {
    println!("Whats your name?");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    println!("Hello, {:}, nice to meet you!", guess.trim());
}
