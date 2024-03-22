use  std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    let mut points = 3;
    println!("Welcome to Rust number guessing game!");
    println!("Guess a number!");
    println!("You have only {} points to guess!", points);
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
   
    loop {
        if points <= 0 {
            println!("Sorry no enough points to guess!");
            break;
        }
        println!("Please enter a number!");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read input");
        
        if guess.trim() == "quit" {
            println!("Thank for playing with me!");
            break;
        }

        let guess : u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
               continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less =>  {
                println!("Too small!");
                points -= 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                points -= 1;
            },
            Ordering::Equal => {
                println!("You get it");
                break;
            },
        }
    }
   
}
