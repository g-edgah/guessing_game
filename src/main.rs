use std::cmp::Ordering;
use std::io;

use rand::RngExt;

fn main() {
    loop {
        println!("guessing game!");

        println!("guess the number(or press Q to quit)");

        let mut guess = String::new();
        let secret: u32 = rand::rng().random_range(1..=100);

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        if guess.trim() == "q" || guess.trim() == "Q" {
            println!("exiting!");
            break;
        }

        // let Ok(guess) = guess.trim().parse::<u32>() 
        //    else {
        //     println!("please type a number");
        //     continue;
        // };

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let Ok(guess) = guess.trim().parse::<u32>() else{}

        //println!("you guessed: {guess}");
        //println!("the secret number is: {secret}");

        // let x = 7;
        // let y = 67;

        // println!("{x} plus {y} is equal to {}", x + y);

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too low");
                println!("try again");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too high");
                println!("try again");
            }
        }
    }
}
