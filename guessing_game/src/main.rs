use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut play_again = true;

    while play_again {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut guessed_correctly = false;

        while !guessed_correctly {
            println!("Please input your guess:");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    guessed_correctly = true;
                }
            }
        }

        println!("Do you want to play again? (yes/no)");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        play_again = answer.trim().to_lowercase() == "yes";
    }
}
