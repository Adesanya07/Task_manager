

pub mod task;
// Compare this snippet from src\main.rs:
pub use task::task::Task;

use::std::io;

use::chrono::prelude::Local;

fn main() {
    println!("Welcome to Rusty Tasks manager!");

    let mut task: Vec<Task> = Vec::new();

    loop {
        println!("command");
        println!("-add <title> <description> <due_date>");
        println!("-view");
        println!("complete <task_index>");
        println!("-filter <completed | upcoming>\\n");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts:  Vec<&str> = input.trim().split_whitespace().collect();

        match parts[0] {
            "-add" => {
                let title = parts[1].to_string();
                let description = parts[2].to_string();
                let due_date = parts[3].parse::<chrono::NaiveDate>().unwrap();
                let new_task = Task::new(title, description, due_date);
                task.push(new_task);
            },
            "-view" => {
                for (index, task) in task.iter().enumerate() {
                    println!("{}: {}", index, task.title);
                }
            },
            "-complete" => {
                let index = parts[1].parse::<usize>().unwrap();
                task[index].completed = true;
            },
            "-filter" => {
                match parts[1] {
                    "completed" => {
                        for task in task.iter().filter(|t| t.completed) {
                            println!("{}", task.title);
                        }
                    },
                    "upcoming" => {
                        let today = Local::today().naive_local();
                        for task in task.iter().filter(|t| t.due_date >= today) {
                            println!("{}", task.title);
                        }
                    },
                    _ => println!("Invalid filter"),
                }
            },
            _ => println!("Invalid command"),
        }


    }

}
