use std::io;
use std::io::Write;
use rand::Rng;

pub fn guess(){
    let mut count = 0;
    let secret_number = rand::thread_rng().gen_range(1..100);
    let number_first = secret_number/10;
    let number_second = secret_number%10;
    println!("Guess the number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        io::stdout().flush().expect("Failed to flush stdout");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number: ");
                io::stdout().flush().expect("Failed to flush stdout");

                continue;
            }
        };
        let guess_first = guess/10;
        let guess_second = guess%10;
        if guess == secret_number {
            println!("You guessed the number in {} attempts!", count);
            io::stdout().flush().expect("Failed to flush stdout");

            break;
        }
        else if guess_first == number_first || guess_second == number_second {
            println!("You are close to the number: ");
            io::stdout().flush().expect("Failed to flush stdout");
            count += 1;

        }
        else {
            println!("Try again: ");
            io::stdout().flush().expect("Failed to flush stdout");
            count += 1;
        }
    }
}