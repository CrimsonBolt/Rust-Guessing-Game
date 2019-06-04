//Simple guessing game

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::SystemTime;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //Debug line
    //println!("The secret number is: {}", secret_number);

    let mut number_of_guesses : u64 = 0;

    //Insert total time start here

    let global_start = SystemTime::now();

    //TODO: create table for recording time taken for each guess

    loop {

        //TODO: Insert current guess timer start here

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        //TODO: End current guess timer here

        println!("You guessed: {}", guess);

        number_of_guesses = number_of_guesses +1;


//print guesses at each stage

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!, you have taken {} guesses\n", number_of_guesses);
            }
            Ordering::Greater => {
                println!("Too big!, you have taken {} guesses\n", number_of_guesses);
            }
            Ordering::Equal => {
                println!("You won!, you took {} guesses", number_of_guesses);

            //Insert total timer end here
            match global_start.elapsed() {
                    Ok(elapsed) => {
                        println!("It took you {} seconds to win.", elapsed.as_secs());
                        let average_per_guess = elapsed.as_secs() / number_of_guesses;
                        println!("It took you an average of {} seconds per guess", average_per_guess)
                    }
                    Err(_e) => {
                        println!("Time error occured")
                    }
                }

                break;
            }
        }
    }

}
