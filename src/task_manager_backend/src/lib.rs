use candid::CandidType;
use serde::{Serialize, Deserialize};
use std::{cell::RefCell, collections::HashMap};
use std::cell::Cell;


// Task struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u64,
    title: String,
    description: String,
    done: bool,
    is_important: bool,
}

// Task Manager struct
thread_local! {
    static TASKS: RefCell<HashMap<u64, Task>> = RefCell::default();
    static NEXT_ID: Cell<u64> = Cell::new(0);
}

#[ic_cdk::update]
// create task
fn create_task(title: String, description: String, is_important: Option<bool>) -> u64 {
    let id = NEXT_ID.with(|id| {
        let next_id = id.get();
        id.set(next_id + 1);
        next_id
    });

    let task = Task {
        id,
        title,
        description,
        is_important: is_important.unwrap_or(false),
        done: false,
    };

    TASKS.with(|tasks| tasks.borrow_mut().insert(id, task));

    id
}

#[ic_cdk::query]
// get task by id
fn get_task(id: u64) -> Option<Task> {
    TASKS.with(|tasks| tasks.borrow().get(&id).cloned())
}

#[ic_cdk::query]
// get all tasks
fn get_all_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().values().cloned().collect())
}

#[ic_cdk::update]
// update task details
fn update_task(id: u64, title: Option<String>, description: Option<String>, done: Option<bool>, is_important: Option<bool>) -> bool {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            if let Some(new_title) = title {
                task.title = new_title;
            }
            if let Some(new_description) = description {
                task.description = new_description;
            }
            if let Some(new_done) = done {
                task.done = new_done;
            }
            task.is_important = is_important.unwrap_or(false);
            true
        } else {
            false
        }
    })
}

#[ic_cdk::update]
// delete task
fn delete_task(id: u64) -> bool {
    TASKS.with(|tasks| tasks.borrow_mut().remove(&id).is_some())
}

#[ic_cdk::query]
// search task by status
fn search_task_by_status(done: bool) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.done == done)
            .cloned()
            .collect()
    })
}
