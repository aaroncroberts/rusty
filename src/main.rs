use std::io;
use log::{info};
use log4rs;

fn main() {

    log4rs::init_file("src/log4rs.yml", Default::default()).unwrap();

    info!("booting up");

    println!("Welcome to Rust!");
    println!("---------------------------");
    println!("<< Guess a number >>");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read input!");

    println!("You guessed: {}", guess);    
}
