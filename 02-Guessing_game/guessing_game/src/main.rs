use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //rand::thread_rng() : random number generator - one that is local to the current thread of execution and seeded by the operating system
    //gen_range() on the random number generator to generate a number between the range
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess (must be a number!)");

        //In Rust, variables are immutable by default. With the keyword mut before the variable name, we can make the variable mutable.
        //String::new() creates a new empty String --> we create a mutable variable that is a String
        let mut guess = String::new();

        //read_line puts what the user types into the string we’re passing it and returns a value (an io::Result, which is an enum with Ok and Err)
        //If this instance of io::Result is an Err value, expect will cause the program to crash and display the message passed in arg
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //We can shadow variable with Rust! It is often use to converting values
        //We bind guess to trim().parse() which deleted any whitespace and parse the String to a number
        //u32 precise that we want a 32-bit integer
        //Parse can fail if the user entered some emoji or stuff like that. That's why we need to raise an exception.
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //we better want the user to type only num so that they play the game
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        //Ordering is an enum with Less, Greater and Equal
        //cmp() compares two values and can be called on anything that can be compared : takes a reference to whatever you want to compare with
        //match expression is used to decide what to do. It is kinda like a connection where there is a pattern and the code would run there if it matches this pattern
            
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}