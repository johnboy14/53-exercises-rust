use std::io;

fn main() {
    let mut i = String::new();
    println!("What is the input string?");
    io::stdin().read_line(&mut i);
    println!("{} has {} characters.", i.trim(), i.trim().chars().count());

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
    Ok(n) => {
        println!("{} bytes read", n);
        println!("{}", input);
    }
    Err(error) => println!("error: {}", error),
}
}
