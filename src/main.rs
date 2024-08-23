mod helloworld;
mod basiccalculator;
mod guessnumber;
mod fileio;
mod menu;

use std::io::{self, Write};
use menu::get_menu_options;

fn read_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
fn main() {
    //Holy shit working with castings and flushes are hard!
    loop{
        let menu_options = get_menu_options();
        for option in &menu_options {
            println!("{} - {}", option.number, option.description);
        }
        print!("Select code snippet: ");
        io::stdout().flush().expect("Failed to flush stdout.");

        let selection_input = read_input();
        let selection_input: u32 = match selection_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                io::stdout().flush().expect("Failed to flush stdout.");
                continue;
            }
        };
        match selection_input{
            1=>{guessnumber::guess();}
            2=>{fileio::read_file();}
            3=>{fileio::write_into_file();}
            4=>{
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

                    let a: i32;
                    let b: i32;

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
            5=>{
                let greeting = helloworld::helloworld();
                println!("{}", greeting);}
            6=>{fileio::append_to_file();}
            7=>{break;}
            _ => {println!("Invalid selection. Please enter a valid selection.");}
        }
    }
}
