# To-Do List App

This is a simple command-line To-Do List application written in Rust. The app allows you to add, view, and remove tasks from your to-do list.

## Features

- Add tasks to the list
- View all tasks
- Remove tasks by their index number
- Simple and easy to use

## Requirements

- Rust (you can install it using [rustup](https://rustup.rs/))

## Installation

1. First, make sure you have Rust installed. If not, install it using:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository (if you have one set up):

   ```bash
   git clone https://github.com/your-username/todo_app.git
   cd todo_app
   ```

3. Or create a new project by running:

   ```bash
   cargo new todo_app
   cd todo_app
   ```

4. Run the project:

   ```bash
   cargo run
   ```

## Usage

When you run the app, you will see a menu with the following options:

```
--- To-Do List Menu ---
1. View tasks
2. Add a new task
3. Remove a task
4. Exit
```

- Select `1` to view all the tasks in your to-do list.
- Select `2` to add a new task.
- Select `3` to remove a task by entering the task number.
- Select `4` to exit the application.

### Example

```bash
cargo run
```

Output:

```
--- To-Do List Menu ---
1. View tasks
2. Add a new task
3. Remove a task
4. Exit
Please choose an option: 2
Enter a new task: Learn Rust
Task added!
```

## Contributing

Feel free to fork this repository and submit pull requests to add new features or fix bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
