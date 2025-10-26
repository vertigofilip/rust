use std::cell::RefCell;
use chrono::{DateTime, Utc};
use std::{io};
use std::process::Command;
use std::rc::Rc;

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

#[derive(PartialEq)]
enum TaskStatusType {
    Todo,
    InProgress,
    Review,
    Testing,
    Done,
    PendingDecision,
    Canceled,
    Structural,
}

struct Task {
    name: String,
    description: String,
    creation_date: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>, // Correct field type for optional datetime
    task_status: TaskStatusType,     // Correct field type for task status
    subtasks: Vec<Rc<RefCell<Task>>>,
    previous: Option<Rc<RefCell<Task>>>,
}

impl Task {
    fn new(name: String, description: String, due_date: Option<DateTime<Utc>>, task_status: TaskStatusType, previous: Option<Rc<RefCell<Task>>>) -> Rc<RefCell<Task>> {
        Rc::new(Self {
            name,
            description,
            creation_date: Utc::now(),
            due_date,
            task_status,
            subtasks: Vec::new(),
            previous,
        }.into())
    }
    fn new_task_list() -> Rc<RefCell<Task>> {
        Rc::new(Self {
            name: "Task List".to_string(),
            description: "This is a task list".to_string(),
            creation_date: Utc::now(),
            due_date: None,
            task_status: TaskStatusType::Structural,
            subtasks: Vec::new(),
            previous: None,
        }.into())
    }
    fn new_from_keyboard(previous: Option<Rc<RefCell<Task>>>) -> Rc<RefCell<Task>> {
        println!("Type task name");
        let mut name_inp = String::new();
        io::stdin().read_line(&mut name_inp).expect("Failed to read line");
        let name_inp = name_inp.trim().to_string();

        println!("Type task description");
        let mut description_inp = String::new();
        io::stdin().read_line(&mut description_inp).expect("Failed to read line");
        let description_inp = description_inp.trim().to_string();

        Self::new(name_inp, description_inp, None, TaskStatusType::Todo, previous)
    }
    fn eddit(&mut self, name_new: String, description_new: String, due_date_new: Option<DateTime<Utc>>, task_status_new: Option<TaskStatusType>) {
        if self.task_status != TaskStatusType::Structural {
            if !name_new.is_empty() {
                self.name = name_new;
            }
            if !description_new.is_empty() {
                self.description = description_new;
            }
            if let Some(new_date) = due_date_new {
                self.due_date = Some(new_date);
            }
            if let Some(new_status) = task_status_new {
                self.task_status = new_status;
            }
        }
    }
    fn eddit_from_keyboard(&mut self) {
        println!("Type task name, or leave emty to not change");
        let mut name_inp = String::new();
        io::stdin().read_line(&mut name_inp).expect("Failed to read line");
        println!("Type task description, or leave emty to not change");
        let mut description_inp = String::new();
        io::stdin().read_line(&mut description_inp).expect("Failed to read line");
        println!("Type task status, or leave emty to not change");
        let mut task_status_inp = String::new();
        io::stdin().read_line(&mut task_status_inp).expect("Failed to read line");
        if task_status_inp.to_lowercase() == "todo" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::Todo))
        }
        else if task_status_inp.to_lowercase() == "done" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::Done))
        }
        else if task_status_inp.to_lowercase() == "in progress" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::InProgress))
        }
        else if task_status_inp.to_lowercase() == "pending decision" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::PendingDecision))
        }
        else if task_status_inp.to_lowercase() == "canceled" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::Canceled))
        }
        else if task_status_inp.to_lowercase() == "review" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::Review))
        }
        else if task_status_inp.to_lowercase() == "testing" {
            self.eddit(name_inp, description_inp, None, Some(TaskStatusType::Testing))
        }
        else {
            self.eddit(name_inp, description_inp, None, None)
        }
    }
    fn display(&self) {
        match self.task_status {
            TaskStatusType::Todo => println!("□ Todo"),
            TaskStatusType::InProgress => println!("◩ Doing"),
            TaskStatusType::Done => println!("■ Done"),
            TaskStatusType::PendingDecision => println!("? Pending decision"),
            TaskStatusType::Canceled => println!("✕ Canceled"),
            TaskStatusType::Review => println!("○ review"),
            TaskStatusType::Testing => println!("◉ testing"),
            TaskStatusType::Structural => println!("- structural"),
        }
        println!("Name:");
        println!("{}", self.name);
        println!("Description:");
        println!("{}", self.description);
        println!("Dreated at:");
        println!("{}", self.creation_date);
        println!("Due date at:");
        match self.due_date {
            Some(date) => println!("Due date: {}", date),
            None => println!("No due date set"),
        }
        println!("Number of subtasks:");
        println!("{}", self.subtasks.len());
    }
    fn display_in_line(&self) {
        match self.task_status {
            TaskStatusType::Todo => print!("□ Todo"),
            TaskStatusType::InProgress => print!("◩ Doing"),
            TaskStatusType::Done => print!("■ Done"),
            TaskStatusType::PendingDecision => print!("? Pending decision"),
            TaskStatusType::Canceled => print!("✕ Canceled"),
            TaskStatusType::Review => print!("○ review"),
            TaskStatusType::Testing => print!("◉ testing"),
            TaskStatusType::Structural => print!("- structural"),
        }
        print!("Name:");
        print!("{} |", self.name);
        print!("Description:");
        print!("{} |", self.description);
        print!("Dreated at:");
        print!("{} |", self.creation_date);
        print!("Due date at:");
        match self.due_date {
            Some(date) => print!("Due date: {} |", date),
            None => print!("No due date set |"),
        }
        print!("Number of subtasks:");
        print!("{} ", self.subtasks.len());
    }
    fn display_recursive(&self, level: u32) {
        if level > 0 {
            for _i in 0..level {
                print!("|");
            }
        }
        self.display_in_line();
        println!();
        for n in &self.subtasks {
            n.borrow().display_recursive(level + 1);
        }
    }
    fn display_with_list (&self) {
        self.display();
        println!();
        self.display_recursive(0);
        println!();
    }
    fn go_up(&self) -> Option<Rc<RefCell<Task>>> {
        self.previous.clone()
    }
    fn go_to(&self, index: usize) -> Option<Rc<RefCell<Task>>> {
        self.subtasks.get(index).cloned()
    }
    
    fn go_to_interactive(&self) -> Option<Rc<RefCell<Task>>> {
        println!("Type number of task to go to (0-{}):", self.subtasks.len().saturating_sub(1));
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        
        match number.trim().parse::<usize>() {
            Ok(n) => {
                match self.subtasks.get(n) {
                    Some(rc_task) => Some(rc_task.clone()),
                    None => {
                        println!("Invalid index");
                        None
                    }
                }
            },
            Err(_) => {
                println!("Invalid number");
                None
            }
        }
    }
    fn add_subtask(&mut self, current: Rc<RefCell<Task>>) {
        let task = Task::new_from_keyboard(Some(current));
        self.subtasks.push(task);
    }
    fn remove_subtask_from_keyboard(&mut self) {
    if self.subtasks.is_empty() {
        println!("No subtasks to remove!");
        println!("Press Enter to continue...");
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).expect("Failed to read line");
        return;
    }
    
    println!("Type number of task to delete (0-{}):", self.subtasks.len().saturating_sub(1));
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    
    match number.trim().parse::<usize>() {
        Ok(n) => {
            if n < self.subtasks.len() {
                let removed_task = self.subtasks.remove(n);
                println!("Removed task: {}", removed_task.borrow().name);
                println!("Press Enter to continue...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).expect("Failed to read line");
            } else {
                println!("Invalid index: {} (must be 0-{})", n, self.subtasks.len().saturating_sub(1));
                println!("Press Enter to continue...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).expect("Failed to read line");
            }
        },
        Err(_) => {
            println!("Invalid number entered!");
            println!("Press Enter to continue...");
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy).expect("Failed to read line");
        }
    }
}
    fn progress(&mut self) {
        self.task_status = match self.task_status {
            TaskStatusType::Todo => TaskStatusType::InProgress,
            TaskStatusType::InProgress => TaskStatusType::Review,
            TaskStatusType::Review => TaskStatusType::Testing,
            TaskStatusType::Testing => TaskStatusType::Done,
            TaskStatusType::Done => TaskStatusType::Done,
            TaskStatusType::PendingDecision => TaskStatusType::PendingDecision, // No change
            TaskStatusType::Canceled => TaskStatusType::Canceled, // No change
            TaskStatusType::Structural => TaskStatusType::Structural, // No change
        }
    }
    fn regress(&mut self) {
        self.task_status = match self.task_status {
            TaskStatusType::Done => TaskStatusType::Testing,
            TaskStatusType::Testing => TaskStatusType::Review,
            TaskStatusType::Review => TaskStatusType::InProgress,
            TaskStatusType::InProgress => TaskStatusType::Todo,
            TaskStatusType::Todo => TaskStatusType::Todo,
            TaskStatusType::PendingDecision => TaskStatusType::PendingDecision, // No change
            TaskStatusType::Canceled => TaskStatusType::Canceled, // No change
            TaskStatusType::Structural => TaskStatusType::Structural, // No change
        }
    }
}


fn main() {
    let list_of_tasks: Rc<RefCell<Task>> = Task::new_task_list(); // Removed mut as warned
    let mut current: Rc<RefCell<Task>> = list_of_tasks.clone();
    loop {
        clear_screen();
        current.borrow().display_with_list();
        println!("Menu");
        println!("1 - go to");
        println!("2 - go up");
        println!("3 - go to top");
        println!("4 - eddit");
        println!("5 - remove");
        println!("6 - add subtask");
        println!("7 - progress");
        println!("8 - regress");
        println!("9 - exit");
        println!("Type number of task to chose function (0-8):");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        
        match number.trim().parse::<usize>() {
            Ok(n) => {
                match n {
                    1 => {
                        // Create a scope to drop the borrow before reassigning
                        let new_task = current.borrow().go_to_interactive();
                        if let Some(task) = new_task {
                            current = task;
                        }
                    },
                    2 => {
                        // Create a scope to drop the borrow before reassigning
                        let parent = current.borrow().go_up();
                        if let Some(p) = parent {
                            current = p;
                        } else {
                            println!("Already at top level");
                        }
                    },
                    3 => {
                        current = list_of_tasks.clone();
                    },
                    4 => {
                        current.borrow_mut().eddit_from_keyboard();
                    },
                    5 => {
                        current.borrow_mut().remove_subtask_from_keyboard();
                    },
                    6 => {
                        current.borrow_mut().add_subtask(current.clone());
                    },
                    7 => {
                        current.borrow_mut().progress();
                    },
                    8 => {
                        current.borrow_mut().regress();
                    },
                    9 => {
                        println!("Exiting...");
                        break;
                    },
                    _ => {
                        println!("Invalid option");
                    },
                }
            },
            Err(_) => {}
        }
    }
}
