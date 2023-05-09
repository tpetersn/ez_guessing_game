use std::io;
use rand::Rng; //random number generator
use std::cmp::Ordering; //comparing numbers with Less, Greater and Equal

fn main() {
    println!("Guess a number between 1 -100");
    println!("Guess a number between 1 -1000000");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //rand::thread_rng functions gives us the particular random number generator were looking for
    //gen_range takes a range expression in form of start..=end

    loop { //loop repeats until you have guessed the right number
        print!("Input your guess.");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        //shadows the previous value of guess...shadowing lets you reuse variable name
        //shadowing is often used to convert a value from one type to another type
        //trim eliminates white space
        //parse converts a string to another type (string to number, u32 is a 32-bit integer)
    
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number){ //comapring guess to random number
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break; //now has a break point for when the correct number is guessed
            }
        }
        //with parse, if a user enters a non-number answer, the program will crash causing the program to stop
    }



}

