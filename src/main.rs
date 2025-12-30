use std::{io::{self, Read, Write}, process::exit};
use colored::*;
use std::fs::OpenOptions;

fn decorate() {
    println!("{} {}{} {}", "\n               MY".bright_red(), "TO".bright_cyan(), "DO".bright_yellow(), "LIST".bright_red());
    println!("{}", "add, show, done, delete, search, save, exit".purple().italic())
}

fn open_file_read() -> io::Result<std::fs::File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todolist.txt")
}

fn open_file_write() -> io::Result<std::fs::File> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("todolist.txt")
}

fn read_file() -> io::Result<Vec<String>> {
    let mut file = open_file_read()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    Ok(lines)
}

fn write_file(list: &Vec<String>) -> io::Result<()> {
    let mut file = open_file_write()?;
    
    for task in list {
        writeln!(file, "{}", task)?;
    }

    Ok(())
}

fn load_into_list(unsaved_tasks_index: &mut usize) -> Vec<String> {
    match read_file() {
        Ok(loaded_list) => {
            if !loaded_list.is_empty() {
                *unsaved_tasks_index = loaded_list.len() - 1;
            }

            return loaded_list;
        }
        
        Err(e) => {
            println!("{}", format!("Error: {e}").bright_red());
            Vec::new() //return an empty list
        }
    }
}

fn save(list: &mut Vec<String>, unsaved_tasks_index: &mut usize) {
    if list.is_empty() {
        println!("List Empty");
        return ();
    }

    match write_file(list) {
        Ok(()) => {
            *unsaved_tasks_index = list.len() - 1;
            println!("{}", "Save Successful".bright_green());
        }
        
        Err(e) => println!("{}", format!("Error: {e}").bright_red())
    }
}

fn show_list(list: &Vec<String>, unsaved_tasks_index: &usize) {
    if list.is_empty() {
        println!("List Empty, type {} to add tasks", "add <task>".bright_cyan());
    } else {
        for i in 0..*unsaved_tasks_index+1 {
            let line = format!("{}. {}", i + 1, list[i]);
            println!("{}", line.bright_green());
        }

        for i in *unsaved_tasks_index+1..list.len() {
            let line = format!("{}. {}", i + 1, list[i]);
            println!("{}", line.bright_red());
        }
    }
}

fn single_line_add(list: &mut Vec<String>, command: &str) {
    let new_task = command.strip_prefix("add ").expect("Failed to strip prefix 'add'");
    list.push(new_task.trim().to_string());
    println!("{}", "Task added successfully!".bright_green());
}

fn multi_line_add(list: &mut Vec<String>) {
    let mut new_task = String::new();
    io::stdin().read_line(&mut new_task).expect("Failed to read task");
    list.push(new_task.trim().to_string());
}

fn mark_done(list: &mut Vec<String>, index: usize, unsaved_tasks_index: &mut usize) {
    list.remove(index); //this will be a valid index since we checked it beforehand

    if index <= *unsaved_tasks_index {
        *unsaved_tasks_index -= 1;
    }

    let og_index = index + 1;
    println!("{}{}", format!("Task {og_index} done! Good job!").bright_yellow(), " (Don't forget to save your changes)".bright_purple());
}

fn parse_integer_and_proceed(list: &mut Vec<String>, number: &str, unsaved_tasks_index: &mut usize) {
    let int_number = number.trim().parse::<usize>();

    match int_number {
        Ok(int_number) => {
            if int_number > 0 && int_number <= list.len() { //int num isnt an index, so it goes from 1 to list.len
                let index = int_number - 1;
                mark_done(list, index, unsaved_tasks_index)
            } else {
                println!("{}", "Enter valid task number".bright_red());
            }
        }

        _ => println!("{}", "Task number not a valid number".bright_red())
    }
}

fn single_line_done(list: &mut Vec<String>, command: &str, unsaved_tasks_index: &mut usize) {
    let number = command.strip_prefix("done ").expect("Failed to strip prefix 'done'");
    parse_integer_and_proceed(list, number, unsaved_tasks_index);
}

fn multi_line_done(list: &mut Vec<String>, unsaved_tasks_index: &mut usize) {
    println!("{}", "\nMy Tasks".bright_yellow());
    show_list(list, unsaved_tasks_index);
    
    print!("{}", "\nEnter task number to mark done: ".bright_yellow());
    io::stdout().flush().expect("Unable to flush stdout");
    
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Unable to read number");

    parse_integer_and_proceed(list, &number, unsaved_tasks_index);
}

fn exit_program(list: &Vec<String>, unsaved_tasks_index: &usize) {
    if !list.is_empty() && *unsaved_tasks_index < list.len() - 1 {
        let mut choice = String::new();

        print!("{}", "You have unsaved tasks, quit anyway? (Y/N) ".bright_red());
        io::stdout().flush().expect("Could not flush out stdout");

        io::stdin().read_line(&mut choice).expect("Could not read choice input");
        
        if choice.trim().eq_ignore_ascii_case("Y") {
            exit(0);
        }

    } else {exit(0)};
}

fn main() {
    decorate();

    let mut unsaved_tasks_index = 0;
    let mut tasks = load_into_list(&mut unsaved_tasks_index);

    let mut input = String::new();
    loop {
        input.clear();

        print!("{}", "\ntodo> ".bright_green());
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut input).expect("Failed to read command");

        let command = input.trim();

        //one-liner tasks
        if command.starts_with("add ") {
            single_line_add(&mut tasks, command);
        } 
        
        else if command.starts_with("done ") {
            single_line_done(&mut tasks, command, &mut unsaved_tasks_index);
        } 
        
        //multi-line tasks
        else {
            match command {
                "add" => multi_line_add(&mut tasks),

                "done" => multi_line_done(&mut tasks, &mut unsaved_tasks_index),

                "show" => show_list(&tasks, &unsaved_tasks_index),
                
                "save" => save(&mut tasks, &mut unsaved_tasks_index),

                "exit" => exit_program(&tasks, &unsaved_tasks_index),

                _ => println!("{}", "Error: Enter valid command".bright_red().italic())
            }
        }
    }
}
