use std::io;
use std::io::Write;
use std::fs::{File, OpenOptions};
use std::io::Read;
use crate::read_input;
use crate::errorlog::log_error;

pub fn read_file() {
    loop {
        print!("Enter file path or ('q') to exit: ");
        io::stdout().flush().unwrap_or_else(|_| log_error("Failed to flush stdout"));

        let file_path = read_input();
        if file_path == "q" {
            break;
        }
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                log_error("File not found. Please enter a valid file path.");
                continue;
            }
        };

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .unwrap_or_else(|_| {
                log_error("Failed to read file");
                0 // Return a default usize value (e.g., 0) on error
            });        println!("File contents:\n{}", file_contents);
        break;
    }
}

fn create_file(path: &str) -> Result<File, io::Error> {
    File::create(path)
}

fn read_message() -> String {
    let mut message = String::new();
    print!("Enter message to write to file: ");
    io::stdout().flush().unwrap_or_else(|_| log_error("Failed to flush stdout"));

    io::stdin().read_line(&mut message)
        .unwrap_or_else(|_| {
            log_error("Failed to read line");
            0 // Return a default usize value (e.g., 0) on error
        });
    return message;
}

pub fn write_into_file() {
    print!("Enter file path to write to: ");
    io::stdout().flush().unwrap_or_else(|_| log_error("Failed to flush stdout"));
    let path = read_input();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => match create_file(&path) {
            Ok(file) => file,
            Err(e) => {
                log_error(format!("Failed to create file: {}", e));
                return;
            }
        },
    };

    let message = read_message();
    file.write_all(message.as_bytes()).unwrap_or_else(|_| log_error("Failed to write to file"));
    println!("Message written to file successfully!");
}

pub fn append_to_file() {
    print!("Enter file path to append to: ");
    io::stdout().flush().unwrap_or_else(|_| log_error("Failed to flush stdout"));
    let path = read_input();

    let mut file = match OpenOptions::new().append(true).open(&path) {
        Ok(file) => file,
        Err(_) => match create_file(&path) {
            Ok(file) => file,
            Err(e) => {
                log_error(format!("Failed to create file: {}", e));
                return;
            }
        },
    };

    let message = read_message();
    file.write_all(message.as_bytes()).unwrap_or_else(|_| log_error("Failed to write to file"));
    println!("Message appended to file successfully!");
}