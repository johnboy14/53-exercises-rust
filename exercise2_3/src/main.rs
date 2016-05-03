use std::io;

fn main() {
    let mut quote = String::new();
    let mut author = String::new();
    println!("What is the quote?");
    io::stdin().read_line(&mut quote);
    println!("Who said it?");
    io::stdin().read_line(&mut author);

    let sentence =  author.trim().to_string() + " says, " + &quote.trim();

    println!("{}", &sentence);
}
