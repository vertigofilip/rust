use chrono::{DateTime, Utc};
use std::{io};
use std::process::Command;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

enum TaskStatusType {
    Todo,
    InProgress,
    Done,
    PendingDecision,
    Canceled,
}

struct Task {
    name: String,
    description: String,
    creation_date: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>, // Correct field type for optional datetime
    task_status: TaskStatusType,     // Correct field type for task status
    subtasks: Vec<Task>,
}

impl Task {
    // Constructor-like method
    fn new_base() -> Self {
        Task {
            name: "ToDo list".to_string(),
            description: "My ToDo list".to_string(),
            creation_date: Utc::now(),
            due_date: None,
            task_status: TaskStatusType::Todo,
            subtasks: Vec::new(),  // Initializing subtasks as empty
        }
    }
    // Constructor-like method
    fn new(name: String, description: String, creation_date: DateTime<Utc>, due_date: Option<DateTime<Utc>>, task_status: TaskStatusType) -> Self {
        Task {
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            creation_date,
            due_date,
            task_status,
            subtasks: Vec::new(),  // Initializing subtasks as empty
        }
    }
    fn display_status(&self) {
        match self.task_status {
            TaskStatusType::Todo => println!("□ Todo"),
            TaskStatusType::InProgress => println!("▣ Doing"),
            TaskStatusType::Done => println!("■ Done"),
            TaskStatusType::PendingDecision => println!("◩ Undecided"),
            TaskStatusType::Canceled => println!("✕ Canceled"),
        }
    }
    fn display_in_line(tsk: &Task) {
        let status = match tsk.task_status {
            TaskStatusType::Todo => "□",
            TaskStatusType::InProgress => "▣",
            TaskStatusType::Done => "■",
            TaskStatusType::PendingDecision => "◩",
            TaskStatusType::Canceled => "✕",
        };
        print!("{} Name: {}|", status, tsk.name);
        print!(" Task due: ");
        match &tsk.due_date {
            Some(date) => print!("{}|", date.format("%Y-%m-%d %H:%M:%S")),
            None => print!("none|"),
        }
        print!(" created at: {}|", tsk.creation_date.format("%Y-%m-%d %H:%M:%S"));
        println!(" {} subtasks|", tsk.subtasks.len())
    }
    fn display_in_line_details(tsk: &Task) {
        let status = match tsk.task_status {
            TaskStatusType::Todo => "□ Todo",
            TaskStatusType::InProgress => "▣ Doing",
            TaskStatusType::Done => "■ Done",
            TaskStatusType::PendingDecision => "◩ Undecided",
            TaskStatusType::Canceled => "✕ Canceled",
        };
        print!("{} Name: {}|", status, tsk.name);
        print!(" Task due: ");
        match &tsk.due_date {
            Some(date) => print!("{}|", date.format("%Y-%m-%d %H:%M:%S")),
            None => print!("none|"),
        }
        print!(" created at: {}|", tsk.creation_date.format("%Y-%m-%d %H:%M:%S"));
        println!(" {} subtasks|", tsk.subtasks.len())
    }
    fn display_details(tsk: &Task) {
        let status = match tsk.task_status {
            TaskStatusType::Todo => "□ Todo",
            TaskStatusType::InProgress => "▣ Doing",
            TaskStatusType::Done => "■ Done",
            TaskStatusType::PendingDecision => "◩ Undecided",
            TaskStatusType::Canceled => "✕ Canceled",
        };
        println!("{}", status);
        println!("Name: {}", tsk.name);
        println!("Description {}", tsk.description);
        print!(" Task due: ");
        match &tsk.due_date {
            Some(date) => print!("{}|", date.format("%Y-%m-%d %H:%M:%S")),
            None => println!("none|"),
        }
        println!(" created at: {}|", tsk.creation_date.format("%Y-%m-%d %H:%M:%S"));
        println!(" {} subtasks|", tsk.subtasks.len())
    }
    fn display_line_recursively(tsk: &Task, spaces: u16){
        for i in 0..spaces{
            if i == spaces - 1{print!("├");}
            else {print!("|");}
        }
        if spaces == 0 {Task::display_details(tsk);}
        else {Task::display_in_line(tsk);}
        for t in &tsk.subtasks{
            Task::display_line_recursively(&t, spaces+1);
        }
    }
}


fn main() {
    let mut tasks: Task = Task::new_base();
    let mut inp = String::new();
    let mut inp2 = String::new();
    let mut display = &mut tasks;
    loop{
        clear_screen();
        Task::display_line_recursively(&display, 0);
        println!("Select Option");
        println!("1 - display all");
        println!("2 - go to");
        println!("3 - go to base");
        println!("4 - add");
        println!("5 - remove");
        println!("6 - edit");
        println!("7 - show more");
        println!("8 - exit");
        inp.clear();
        inp2.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        if inp.contains('1'){
            println!("na");
        }
        if inp.contains('2'){
            println!("Select number of task to select");
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            if let Ok(index) = inp2.trim().parse::<usize>() {
                if index < display.subtasks.len() {
                    display = &mut display.subtasks[index];
                } else {
                    println!("Index out of bounds.");
                }
            } else {
                println!("Please enter a valid number.");
            }
        }
        if inp.contains('3'){
            display = &mut tasks;
        }
        if inp.contains('4'){
            println!("Write a name:");
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            let new_name = inp2.clone();
            inp2.clear();
            println!("Write a description:");
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            let new_description = inp2.clone();
            inp2.clear();
            display.subtasks.push(Task::new(new_name.to_string(), new_description.to_string(), Utc::now(), None, TaskStatusType::Todo));
        }
        if inp.contains('5'){
            println!("Select number of task to delete");
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            if let Ok(index) = inp2.trim().parse::<usize>() {
                if index < display.subtasks.len() {
                    display.subtasks.remove(index);
                } else {
                    println!("Index out of bounds.");
                }
            } else {
                println!("Please enter a valid number.");
            }
        }
        if inp.contains('6'){
            println!("Change name {} to, or leave empty:", display.name);
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            display.name = inp2.clone().trim().to_string();
            inp2.clear();
            println!("Change description {} to, or leave empty:", display.description);
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            display.description = inp2.clone().trim().to_string();
            inp2.clear();
            print!("Change task status ");
            display.display_status();
            println!(" to, or leave empty:");
            println!("1 - □ Todo");
            println!("2 - ▣ Doing");
            println!("3 - ■ Done");
            println!("4 - ◩ Undecided");
            println!("5 - ✕ Canceled");
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            if inp2.contains('1'){
                display.task_status = TaskStatusType::Todo;
            }
            else if inp2.contains('2'){
                display.task_status = TaskStatusType::InProgress;
            }
            else if inp2.contains('3'){
                display.task_status = TaskStatusType::Done;
            }
            else if inp2.contains('4'){
                display.task_status = TaskStatusType::PendingDecision;
            }
            else if inp2.contains('5'){
                display.task_status = TaskStatusType::Canceled;
            }
        }
        if inp.contains('8'){
            break;
        }
        println!("Press enter to continue.");
        io::stdin()
            .read_line(&mut inp2)
            .expect("Failed to read line");
    }
}
