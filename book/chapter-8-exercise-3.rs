use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    let mut data = HashMap::new();

    println!("+----------------------------------------------+");
    println!("| Rust Book Text interface                     |");
    println!("| (╯°□°)╯︵ ┻━                                 |");
    println!("+----------------------------------------------+");
    println!("| Commands:                                    |");
    println!("+----------------------------------------------+");
    println!("| Add (person) to (department)                 |");
    println!("| List                                         |");
    println!("| Quit                                         |");
    println!("+----------------------------------------------+");
    println!("| by Lucas Coelho.                             |");
    println!("+----------------------------------------------+");

    loop {
        io::stdin().read_line(&mut input).expect("Invalid input");
        let arguments: Vec<&str> = input.split(' ').collect();
        let command: &str = &arguments[0].trim_end_matches("\n");
        let command = command.to_lowercase();

        match command.as_str() {
            "add" => add_employee(&mut data, &arguments),
            "list" => list_employees(&data),
            "quit" => break,
            _ => println!("Invalid command."),
        }

        input.clear();
        continue;
    }
}

fn add_employee(data: &mut HashMap<String, Vec<String>>, parameters: &Vec<&str>) {
    const ARGS_SIZE: usize = 4;

    if parameters.len() < ARGS_SIZE {
        println!("Invalid amount of parameters. Expected {}", ARGS_SIZE);
        return;
    }

    let name: String = String::from(parameters[1]);
    let department: String = String::from(parameters[3].trim_end_matches("\n"));

    if let Some(vec) = data.get_mut(&department) {
        println!("Inserted {}", &name);
        vec.insert(vec.len() - 1, name);
        vec.sort();
    } else {
        println!("Inserted {}", &name);
        data.entry(department).or_insert(vec![name]);
    }
}

fn list_employees(data: &HashMap<String, Vec<String>>) {
    for (department, employees) in data {
        println!("{}", department.to_uppercase());

        for _e in employees {
            println!("{}", _e);
        }

        println!("");
    }
}
