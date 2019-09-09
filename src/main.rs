use std::{
    io,
    cmp::Ordering
};
use rand::Rng;
mod score;
use score::scoring;

fn main() {
    println!("Guess the Number!");

    let rand_number = rand::thread_rng().gen_range(1, 101);

    println!("*whisper* the random number is {}", rand_number);

    let mut guess_list: Vec<u32> = Vec::new();
    for _number in 0..5{
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
    let score = scoring::get_score(guess_list, rand_number);

    println!("Your score is {}", score);

}
