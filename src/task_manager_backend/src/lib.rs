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
}

// Task Manager struct
thread_local! {
    static TASKS: RefCell<HashMap<u64, Task>> = RefCell::default();
    static NEXT_ID: Cell<u64> = Cell::new(0);
}

#[ic_cdk::update]
// create task
fn create_task(title: String, description: String) -> u64 {
    let id = NEXT_ID.with(|id| {
        let next_id = id.get();
        id.set(next_id + 1);
        next_id
    });

    let task = Task {
        id,
        title,
        description,
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
// update task status
fn update_task_status(id: u64, title: String, description: String, done: bool) -> bool {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.title = title;
            task.description = description;
            task.done = done;
            true
        } else {
            false
        }
    })
}

#[ic_cdk::update]
// update either task title and description
fn update_task(id: u64, title: Option<String>, description: Option<String>) -> bool {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            if let Some(new_title) = title {
                task.title = new_title;
            }
            if let Some(new_description) = description {
                task.description = new_description;
            }
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