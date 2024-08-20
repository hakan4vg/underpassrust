//get a file path from user and read the file, output the content of the file to the console

use std::io;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use crate::read_input;

pub fn read_file(){
    loop {
        print!("Enter file path or ('q') to exit: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let file_path = read_input();
        if file_path == "q" {
            break;
        }
        let mut file = match File::open(&file_path){
            Ok(file) => file,
            Err(_) => {
                println!("File not found. Please enter a valid file path.");
                continue;
            }
        };

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).expect("Failed to read file");
        println!("File contents:\n{}", file_contents);
        break;

    }

}

fn create_file(path: &str) -> File {
    let file = File::create(path).expect("Failed to create file");
    return file;

}
fn read_message() -> String {
    let mut message = String::new();
    print!("Enter message to write to file: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut message).expect("Failed to read line");
    return message;
}
pub fn write_into_file(){
    print!("Enter file path to write to: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let path = read_input();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => create_file(&path),
    };

    let message = read_message();
    file.write_all(message.as_bytes()).expect("Failed to write to file");
    println!("Message written to file successfully!");
}