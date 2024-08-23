use std::fs::{OpenOptions, File};
use std::io::Write;
use std::fmt::Display;
use std::time::SystemTime;

pub fn log_error<T: Display>(error: T) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
        .expect("Failed to open error.log");

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get time")
        .as_secs();

    let error_message = format!("{}: {}\n", time, error);
    file.write_all(error_message.as_bytes())
        .expect("Failed to write to error.log");
    print!("{}", error_message);
}