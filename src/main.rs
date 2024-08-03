mod helloworld;
mod basiccalculator;

use std::io::{self, Write};

fn read_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
fn main() {
    let greeting = helloworld::helloworld();
    println!("{}", greeting);


    //Holy shit working with castings and flushes are hard!
    loop {
        print!("Enter operation (+, -, *, /, %, ^, !, q to quit): ");
        io::stdout().flush().expect("Failed to flush stdout");


        let input = read_input();
        if input.is_empty() {
            println!("No input provided. Please enter a valid operation.");
            continue;
        }

        let operation = input.chars().next().unwrap();

        if operation == 'q' {
            break;
        }

        let mut a: i32;
        let mut b: i32;

        if operation != '!' {
            print!("Enter first number: ");
            io::stdout().flush().expect("Failed to flush stdout");
            a = read_input().parse().expect("Please enter a valid number");

            print!("Enter second number: ");
            io::stdout().flush().expect("Failed to flush stdout");
            b = read_input().parse().expect("Please enter a valid number");
        } else {
            print!("Enter number: ");
            io::stdout().flush().expect("Failed to flush stdout");
            a = read_input().parse().expect("Please enter a valid number");
            b = 0; // b is not used in factorial
        }

        let result:f32 = match operation {
            '+' => basiccalculator::add(a, b) as f32,
            '-' => basiccalculator::subtract(a, b) as f32,
            '*' => basiccalculator::multiply(a, b) as f32,
            '/' => basiccalculator::divide(a, b),
            '%' => basiccalculator::modulo(a, b) as f32,
            '^' => basiccalculator::power(a, b) as f32,
            '!' => basiccalculator::factorial(a) as f32,
            _ => {
                println!("Invalid operation");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
