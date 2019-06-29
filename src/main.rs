//Simple guessing game
use std::time::SystemTime;
use std::io;
use std::cmp::Ordering;
use guessing_game::initialize_game;
use guessing_game::player_guess;

fn main() {
    let secret_number = initialize_game();

    //Total time start here. Also initialize the time taken per guess timetable
    let global_start = SystemTime::now();
    let mut guess_time_vec = Vec::new();
    //Initialize guess counter
    let mut number_of_guesses : u64 = 0;

    

    //Debug line
    //println!("The secret number is: {}", secret_number);

    loop {

        let guess_start_time = SystemTime::now();

        let mut guess = player_guess();  

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
                continue;
            }
            Ordering::Greater => {
                println!("Too big!, you have taken {} guesses\n", number_of_guesses);
                continue;
            }
            Ordering::Equal => {
                match global_start.elapsed() {
                    Ok(elapsed) => {
                        let average_per_guess = elapsed.as_secs() / number_of_guesses;
                        println!("You won!, you took {} guesses", number_of_guesses);
                        println!("It took you {} seconds to win.", elapsed.as_secs());
                        println!("It took you an average of {} seconds per guess", average_per_guess);
                        break;
                    }
                    Err(_e) => {
                        println!("Time error occured")
                    }
                }
            }                                                        
        }
    }
        
                
    println!("Do you want to see the time taken for each guess?
     y will display a table
     n will close the game");
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
        println!("Exiting game.");
    }

    println!("Press any key to exit");
}
