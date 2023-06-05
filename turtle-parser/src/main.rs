use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Command {
    cmd: char,
    has_arg: bool,
    func: fn(name: Option<&str>, argument: Option<&str>),
}


pub fn select_pen(name: Option<&str>, argument: Option<&str>) {}
pub fn pen_up(name: Option<&str>, argument: Option<&str>) {}
pub fn pen_down(name: Option<&str>, argument: Option<&str>) {}
pub fn pen_move(name: Option<&str>, argument: Option<&str>) {
    if let (Some(name), Some(argument)) = (name, argument) {
        println!("Moving pen\nDirection: {}\nAmount: {}", name, argument)
    }
}

const COMMANDS: [Command; 7] = [
    Command { cmd: 'P', has_arg: true, func: select_pen },
    Command { cmd: 'U', has_arg: false, func: pen_up },
    Command { cmd: 'D', has_arg: false, func: pen_down },
    Command { cmd: 'N', has_arg: true, func: pen_move },
    Command { cmd: 'E', has_arg: true, func: pen_move },
    Command { cmd: 'S', has_arg: true, func: pen_move },
    Command { cmd: 'W', has_arg: true, func: pen_move },
];

fn parse_line(line: &String) {
    let mut values = line.split_whitespace();
    let command_name: Option<&str> = values.next();
    let command_arg: Option<&str> = values.next();
    
    if let (Some(value), Some(argument)) = (command_name, command_arg) {
        let index = COMMANDS.iter().position(|cmd| cmd.cmd.to_string() == value).unwrap();
        (COMMANDS[index].func)(Some(value), Some(argument));
    } else if let Some(value) = command_name {
        let index = COMMANDS.iter().position(|cmd| cmd.cmd.to_string() == value).unwrap();

        if COMMANDS[index].has_arg {
            println!("Missing parameter for command '{}'", value)
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./sample") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                parse_line(&ip);
            }
        }
    }
    println!("Hello, world!");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}