use std::io::*;
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let input = parts.next().unwrap();
        let args = parts;

        match input {
            input => {
                let mut command = Command::new(input).args(args).spawn().unwrap();
                command.wait().unwrap();
            }
        }
    }
}
