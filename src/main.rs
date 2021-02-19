extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN_NUM: u32 = 1;
const MAX_NUM: u32 = 10;

fn main() {
    let mut rng = thread_rng();
    let stdin = io::stdin();

    // Generate random number
    let target_num: u32 = rng.gen_range(MIN_NUM..=MAX_NUM);

    let mut has_guessed_correct = false;
    let mut guesses: usize = 0;

    println!(
        "Generated number between {} and {} (inclusive).\nStart guessing!",
        MIN_NUM, MAX_NUM
    );

    while !has_guessed_correct {
        // Get user guess
        let mut input_buf = String::new();
        let _ = stdin.read_line(&mut input_buf);
        let input = input_buf.trim();

        match input.parse::<u32>() {
            Ok(guess) => {
                guesses += 1;

                match guess.cmp(&target_num) {
                    Ordering::Equal => {
                        has_guessed_correct = true;
                    }
                    Ordering::Less => {
                        println!("Guessed LESS THAN random number!");
                    }
                    Ordering::Greater => {
                        println!("Guessed GREATER THAN random number!");
                    }
                }
            }
            Err(_) => eprintln!("Invalid input: {}", &input),
        }
    }

    // WIN! Guessed correct number!
    println!("You did it!\nIt took you {} guesses.", guesses);
}
