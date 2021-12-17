use rand::Rng;
use std::cmp::Ordering;
use std::io;


// A simple Number Guessing Game lol :)`
// Rust comes with immutable variables :)

fn main() {
    println!("GUESS THE NUMBER");

    let _secret_number = rand::thread_rng().gen_range(1..101);

     // println!("The Secret Number is : {}", _secret_number); 

    loop {                                                       // four indents is require to run the loop
        println!("Please Input Your Guess 🤔");

            let mut _guess = String::new();
            let _stdin = io::stdin();
            _stdin.read_line(&mut _guess).expect("Failed to read sign");

            let _guess: u32 =  match _guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You Guessed: {}", _guess);

            match _guess.cmp(&_secret_number) {
            Ordering::Less => println!("To Small 😓"),
            Ordering::Greater => println!("Too Big 😬"),
            Ordering::Equal => {
                println!("You Win 🎉");
                break;
            }        
            
        }
    }
}
