# jsonrepl

A simple interactive CLI tool for validating, exploring, and editing JSON data.

## Requierments

Core Functionality

- [ ] Load in JSON file
- [ ] Validate JSON
- [ ] Output JSON in a human-readable (indented) form
- [ ] Write variables to a file

Interaction & Manipulation

- [ ] Interact with the JSON (read, make immutable transformations, set data to new variables)
- [ ] Support multiple variables
- [ ] Type information (commands to inspect the type of a value or variable)
- [ ] JSON querying/search
- [ ] Immutable transformations

History, Undo & Advanced I/O

- [ ] Command history and undo (allow undoing the last transformation or action)
- [ ] Import/export to and from different file formats

Advanced Data Operations

- [ ] Diff and patch (show differences between two JSON variables or files, and apply patches)
- [ ] Pipelines/Chaining (allow chaining commands, e.g., filter then map in a single line)

Automation & Extensibility

- [ ] Scripting (load commands from a file and run them/batch processing)
- [ ] Git integration (version JSON files with Git, allowing you to track and revert changes)
- [ ] Plugin system (allow users to add custom transformations as Rust plugins or WebAssembly modules)

## Furute Features

- [ ] Validate JSON files and strings
- [ ] Interactive REPL (Read-Eval-Print Loop) for querying and modifying JSON
- [ ] Pretty-print and save your JSON
- [ ] Basic commands: `load`, `validate`, `get`, `set`, `print`, `save`, `exit`

### Build & Run

```sh
cargo build --release
cargo run
```
