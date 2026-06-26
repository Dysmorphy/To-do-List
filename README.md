# To-do List

A minimal CLI to-do list written in Rust. Tasks persist between sessions via local JSON storage.

## Installation

```bash
git clone https://github.com/Dysmorphy/To-do-List
cd To-do-List
cargo build --release
cp target/release/learning_rust /usr/local/bin/todo
```

## Usage

```bash
todo add <task>     # Add a new task
todo list           # List all tasks
todo rm <number>    # Remove a task by number
todo --help         # Show help
```

## Example

```bash
$ todo add "Buy groceries"
Task successfully added

$ todo add "Read the Rust Book"
Task successfully added

$ todo list
List of current tasks:

1) Buy groceries
2) Read the Rust Book

$ todo rm 1
Task removed

$ todo list
List of current tasks:

1) Read the Rust Book
```

## Storage

Tasks are saved to `~/.todo/tasks.json` and persist between sessions.

## Built With

- [Rust](https://www.rust-lang.org/)
- [serde / serde_json](https://serde.rs/) — serialization
- [colored](https://crates.io/crates/colored) — terminal colors
- [dirs](https://crates.io/crates/dirs) — platform-aware home directory
