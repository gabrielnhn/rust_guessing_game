use std::io;
use std::io::Write;
use rand::Rng;
// use typenum;
// use atoi::FromRadix10;

fn main() {
    
    let secret: i32 = rand::thread_rng().gen_range(1..=100);
    
    // println!("Guess the number ({secret})!");
    println!("Guess the number!");
    let mut guess_num: i32 = -1;
    let mut guess = String::new();
    
    while guess_num != secret
    {   
        guess.clear();
        print!("Your guess: ");
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        guess = guess.trim().to_string();

        guess_num = guess.parse::<i32>().expect("Inp is not a number");


        if guess_num > secret
        {
            println!("Lower...");
        }
        else if guess_num < secret
        {
            println!("Higher...");
        }
        else
        {
            println!("You're right!");
        }
        
        io::stdout().flush().expect("Could not flush stdout");
        
    }

}