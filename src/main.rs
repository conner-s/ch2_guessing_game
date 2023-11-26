mod score;
use std::{
    io,
    cmp::Ordering
};
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    const MAX_GUESSES: u32 = 5;

    let rand_number = rand::thread_rng().gen_range(1..=100);
    println!("*whisper* the random number is {}", rand_number);
    let mut guess_list: Vec<u32> = Vec::new();
    guess_list = get_guesses(guess_list, MAX_GUESSES, &rand_number);
    println!("{:?}", guess_list);
    let score = score::get_score(guess_list, rand_number);

    println!("Your score is {}", score);

}

//function that takes a mut Vec<u32> as input and outputs a Vec<u32>
//this function will take in the user's guesses and return them as a vector
//this function will also take in the MAX_GUESSES constant
/// # Examples
/// ```
/// let mut guess_list: Vec<u32> = Vec::new();
/// guess_list = get_guesses(guess_list, MAX_GUESSES);
/// ```
/// # Panics
/// This function will panic if the user does not enter a number
 fn get_guesses(mut guess_list: Vec<u32>, max_guesses: u32, rand_number: &u32) -> Vec<u32> {
    for _number in 0..max_guesses{
        let mut guess = String::new();
        println!("Please enter guess #{}", _number+1);
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Not a number");

        match guess.cmp(&rand_number){
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => println!("Just right"),
            Ordering::Less => println!("Too low"),
        }
        guess_list.push(guess)
    }
    guess_list
 }