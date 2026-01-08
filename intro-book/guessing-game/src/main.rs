use std::io;

fn main() {
    println!("Let's play the guessing game !!!!!");
    println!("Guess a number : ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could'nt read line ");

    println!("Im guessing the number ... {guess}")
}
