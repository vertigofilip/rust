use chrono::{DateTime, Utc};
use std::{io};

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
    fn newBase() -> Self {
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
            name,
            description,
            creation_date,
            due_date,
            task_status,
            subtasks: Vec::new(),  // Initializing subtasks as empty
        }
    }
}
fn displayInLine(tsk: &Task) {
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
fn displayLineRecursively(tsk: &Task, spaces: u16){
    for i in 0..spaces{
        if i == spaces - 1{print!("├");}
        else {print!("|");}
    }
    displayInLine(tsk);
    for t in &tsk.subtasks{
        displayLineRecursively(&t, spaces+1);
    }
}
fn main() {
    let mut tasks: Task = Task::newBase();
    let mut inp = String::new();
    let mut inp2 = String::new();
    let mut display = &tasks;
    let mut dispMode: bool = false;
    loop{
        displayLineRecursively(&display, 0);
        println!("Select Option");
        println!("1 - display all/display curet");
        println!("2 - go to");
        println!("3 - go to base");
        println!("4 - add");
        println!("5 - remove");
        println!("6 - edit");
        println!("7 - show");
        println!("8 - exit");
        inp.clear();
        inp2.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        if inp.contains('1'){
            if dispMode {dispMode = false;}
            else {dispMode = true;}
        }
        if inp.contains('2'){
            io::stdin()
                .read_line(&mut inp2)
                .expect("Failed to read line");
            if let Ok(index) = inp2.trim().parse::<usize>() {
                if index >= 0 && index < display.subtasks.len() {
                    display = &display.subtasks[index];
                } else {
                    println!("Index out of bounds.");
                }
            } else {
                println!("Please enter a valid number.");
            }
        }
        if inp.contains('3'){
            display = &tasks;
        }
        if inp.contains('4'){
            println!()
        }
        if inp.contains('8'){
            break;
        }
    }
}
