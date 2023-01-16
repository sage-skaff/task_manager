use std::io;

fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    // prompt user to add a task to the task list
    println!("Enter a task title:");

    let mut title = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter a task description:");

    let mut description = String::new();

    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    add_task(&mut task_list, title, description);
    // prompt user to mark a task as complete
    // println!("Enter the index of the task to mark as complete:");

    display_tasks(&mut task_list);
}
fn add_task(task_list: &mut Vec<Task>, title: String, description: String) {
    let new_task = Task {
        title: title,
        description: description,
        status: TaskStatus::Incomplete,
    };
    task_list.push(new_task);
}
// fn mark_complete(task_list: &mut Vec<Task>, index: usize) {
//     task_list[index].status = TaskStatus::Complete;
// }
fn display_tasks(task_list: &mut Vec<Task>) {
    for task in task_list.iter() {
        match task.status {
            TaskStatus::Incomplete => println!("[ ] {} - {}", task.title, task.description),
            TaskStatus::Complete => println!("[X] {} - {}", task.title, task.description),
        }
    }
}
enum TaskStatus {
    Incomplete,
    Complete,
}
struct Task {
    title: String,
    description: String,
    status: TaskStatus,
}
