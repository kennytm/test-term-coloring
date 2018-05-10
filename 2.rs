use std::process::Command;

fn main() {
    println!("Result: {:?}", Command::new(".\\1.exe").status().unwrap());
}
