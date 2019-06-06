//Simple guessing game

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::time::SystemTime;

fn main() {
    println!("Guess the number!");
    //Generate random number and initialize guess counter
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_guesses : u64 = 0;
    //Total time start here. Also initialize the time taken per guess timetable
    let global_start = SystemTime::now();
    let mut guess_time_vec = Vec::new();

    //Debug line
    //println!("The secret number is: {}", secret_number);

    loop {

        let guess_start_time = SystemTime::now();

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_start_time.elapsed() {
            Ok(elapsed) => {
                println!("You guessed: {}, it took {} seconds", guess, elapsed.as_secs());
                guess_time_vec.push(elapsed.as_secs())
            }
            Err(_e) => {
                println!("A time error occured")
            }
        }

        number_of_guesses += 1;

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

                println!("Do you want to see the time taken for each guess? y/n");
                let mut bool_response = String::new();
                
                io::stdin().read_line(&mut bool_response).expect("failed to read string");

                //TODO match response to create timetable
                let mut x = 1;
                if bool_response.trim() == "y" {
                    for i in guess_time_vec{
                        println!("Guess {} took {} seconds", x , i);
                        x += 1;
                    }
                } else{
                    break;
                }

                break;
            }
        }
    }

}
