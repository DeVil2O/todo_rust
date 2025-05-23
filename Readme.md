# Rust Todo Application

A simple command-line Todo application written in Rust that allows you to manage your tasks efficiently.

## Features

- Add new todos with unique IDs
- View all todos
- Get a specific todo by ID
- Update todo titles
- Mark todos as completed/uncompleted
- Delete todos

## Project Structure

The application is built with a simple structure:
- `Todo` struct to represent each todo item with:
  - `id`: Unique identifier
  - `title`: Task description
  - `completed`: Completion status

## Available Functions

- `add_todo`: Add a new todo to the list
- `get_todo`: Retrieve a specific todo by ID
- `update_todo`: Update a todo's title
- `update_todo_status`: Mark a todo as completed or uncompleted
- `delete_todo`: Remove a todo from the list

## Usage

To run the application:

```bash
cargo run
```

Example usage in code:

```rust
let mut todos = Vec::new();

// Add a new todo
add_todo(&mut todos, "Buy groceries".to_string());

// Get a todo
let todo = get_todo(&todos, "1".to_string());

// Update a todo
update_todo(&mut todos, "1".to_string(), "Buy groceries and milk".to_string());

// Mark as completed
update_todo_status(&mut todos, "1".to_string(), true);

// Delete a todo
delete_todo(&mut todos, "1".to_string());
```

## Requirements

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository
2. Navigate to the project directory
3. Run `cargo build` to compile the project
4. Run `cargo run` to start the application

## Future Improvements

- Add persistence (save todos to a file)
- Add due dates for todos
- Add priority levels
- Add categories/tags for todos
- Add command-line interface for user interaction
