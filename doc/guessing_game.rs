use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{

    // create random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to guessing game!");

    loop 
    {
        
        // init your guess
        let mut guess = String::new();
        
        println!("Please enter input your guess: ");

        // read your input and keep its to guess
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // convert guess(string) to int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        // check 
        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("less than"),
            Ordering::Equal => {
                println!("You won");
                break;
            },
            Ordering::Greater => println!("Greater than"),
        }

    }
}
