use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

pub fn run() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();

    println!("Guess the number {}!", secret_number);
    
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
    
        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_int.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        
        guess.clear();
    }
}
