fn main() {
    use std::io::{stdin};
    println!("what is your name?");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Did not enter a correct string");
    println!("your name is {}", name);
}
