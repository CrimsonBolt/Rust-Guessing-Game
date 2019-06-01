//Simple guessing game
//TODO: Attempt to time each guess and give stats at the end of the game for how long the game took to complete.

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::SystemTime;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //Debug line
    //println!("The secret number is: {}", secret_number);

    let mut number_of_guesses : u32 = 0;

    //Insert total time start here

    let start = SystemTime::now();

    //create table for recording time taken for each guess

    loop {

        //Insert current guess timer start here

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        //End current guess timer here

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
            match start.elapsed() {
                    Ok(elapsed) => {
                        println!("It took you {} seconds to win", elapsed.as_secs())
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
