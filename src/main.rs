use std::io::{self, Read, Write};
use colored::*;
use std::fs::OpenOptions;

fn decorate() {
    println!("{} {}{} {}", "\n                 MY".bright_red(), "TO".bright_cyan(), "DO".bright_yellow(), "LIST".bright_red());
    println!("{}", "add, show, done, delete, search, save, load, exit".purple().italic())
}

fn open_file() -> io::Result<std::fs::File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todolist.txt")
}

fn read_file() -> io::Result<Vec<String>> {
    let mut file = open_file()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    Ok(lines)
}

fn show_list(list: &Vec<String>) {
    for i in 0..list.len() {
        println!("{}{} {}", i.to_string().bright_blue(), ")".bright_blue(), list[i]);
    }
}

fn main() {
    decorate();

    let mut tasks: Vec<String> = Vec::new();

    let mut input = String::new();
    loop {
        input.clear();

        print!("{}", "\ntodo> ".bright_green());
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        
        let command = input.trim();
        match command {
            "add" => {
                let mut new_task = String::new();
                io::stdin()
                    .read_line(&mut new_task)
                    .expect("Failed to read task");
                
                tasks.push(new_task);
            }

            "load" => {
                match read_file() {
                    Ok(loaded_list) => {
                        println!("{}", "\nList Loaded successfully from file\n".bright_yellow());

                        if loaded_list.is_empty() {
                            println!("List Empty, type {} to add tasks", "add <task>".bright_cyan());

                        } else {show_list(&loaded_list);};
                    }
                    Err(e) => println!("Error {e}")
                }
            }

            "exit" => break,
            _ => println!("{}", "Error: Enter valid command\n".bright_red().italic())
        }
    }
}
