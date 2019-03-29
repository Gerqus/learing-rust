use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let number_to_guess: u8 = rand::thread_rng().gen_range(1, 101);

    let mut guess_input;
    let mut guess: u8;

    println!("Guess the number!");

    loop {
        guess_input = String::new();

        println!("Please input your guess:");

        io::stdin().read_line(&mut guess_input)
            .expect("Failed to read line");
        
        guess = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(& number_to_guess) {
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("You guessed!");
                break;
            },
            Ordering::Greater => println!("Too big..."),
        }
    }
}