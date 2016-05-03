use std::io;

fn main() {
    let mut noun = String::new();
    let mut verb = String::new();
    let mut adjective = String::new();
    let mut adverb = String::new();

    println!("Enter a noun");
    io::stdin().read_line(&mut noun);
    println!("Enter a verb");
    io::stdin().read_line(&mut verb);
    println!("Enter a adjective");
    io::stdin().read_line(&mut adjective);
    println!("Enter a adverb");
    io::stdin().read_line(&mut adverb);

    println!("Do you {} your {} {} {}, That's hillarious", verb.trim(), adjective.trim(), noun.trim(), adverb.trim());
}
