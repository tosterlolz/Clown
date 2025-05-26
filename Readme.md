# Clown

**Clown** is a simple build automation tool inspired by Make, using a TOML-based configuration file (`Clownfile`).  
It supports dependencies, variables, colored output, and clear targets for building, running, cleaning, and testing your project.

---

## Features

- **TOML-based Clownfile**: clean, modern, and readable configuration.
- **Targets with dependencies**: like in Make, you can specify what depends on what.
- **Variables**: define once, use everywhere with `${var}` syntax.
- **Colored output**: for better visibility of build steps and errors.
- **Friendly CLI**: list targets, get help, and run specific tasks.

---

## Example `Clownfile`

```toml
[vars]
profile = "release"
name = "Clown"

[all]
desc = "Build and run everything"
deps = ["build", "run"]

[build]
desc = "Build the project"
steps = [
    "cargo build --${profile}",
    "echo Build done!"
]

[run]
desc = "Run the project"
steps = ["cargo run --${profile}"]

[clean]
desc = "Clean the project"
steps = ["cargo clean"]

[test]
desc = "Run tests"
deps = ["build"]
steps = ["cargo test"]
```

---

## Usage

1. **Build Clown:**
   ```sh
   cargo build --release
   ```

2. **Run tasks in your project:**
   - **Default (`all`):**
     ```sh
     ./target/release/clown
     ```
   - **Specific target (e.g. `build`):**
     ```sh
     ./target/release/clown build
     ```
   - **List all targets:**
     ```sh
     ./target/release/clown --list
     ```
   - **Show help:**
     ```sh
     ./target/release/clown --help
     ```

---

## How it works

- Reads `Clownfile` from the project root.
- Expands variables like `${profile}` in your steps.
- Resolves dependencies before executing each step.
- Prints each command before it runs, in color.
- Stops and prints an error if any step fails.

---

## Installation

Clone and build from source:

```sh
git clone https://github.com/tosterlolz/clown.git
cd clown
cargo build --release
cp target/release/clown /usr/local/bin/
```

or via `AUR`
```sh
yay -S clown-git
```

---

## License

MIT

---

## Contributing

Pull requests and suggestions are welcome! If you have ideas for new features or improvements, feel free to open an issue.
