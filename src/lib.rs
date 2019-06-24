use rand::Rng;

pub fn initialize_game() -> u64 {
    println!("Guess the number!");
    rand::thread_rng().gen_range(1, 101)
}