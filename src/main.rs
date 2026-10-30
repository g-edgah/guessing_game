use std::io;

fn main() {
    println!("guessing game!");

    println!("guess the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {guess}");

    // let x = 7;
    // let y = 67;

    // println!("{x} plus {y} is equal to {}", x + y);
}
