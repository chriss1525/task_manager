# task_manager

This Rust-based Canister provides a simple task management system, allowing you to create, retrieve, update, and delete tasks. It uses the Internet Computer Candid interface for communication and ic_cdk for initialization.

## Initialization

### clone the Repository
```sh
git clone https://github.com/chriss1525/task-manager.git
cd task-manager
```

### Install Dependencies

#### Rust and ic_cdk
Ensure you have Rust and the Internet Computer Canister Development Kit (ic_cdk) installed.

Find Rust infrastructure and installation instructions [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env)

### Run Internet Computer

Start the Internet Computer Replica

```sh
dfx start --clean --background
```

Deploy the Task Manager Canister

```sh
dfx deploy task_manager_backend
```

This will deploy the Task Manager Canister, and you will receive output containing information about the deployed canister and its identifiers.

