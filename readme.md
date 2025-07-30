# gitlist

A simple command-line tool to recursively find and list Git repositories within a specified directory.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

With Rust installed, you can install `gitlist` using `cargo`:

```bash
cargo install --path .
```

This will build the binary and place it in your Cargo binary directory (e.g., `~/.cargo/bin`), making it available in your shell.

## Usage

Once installed, you can run `gitlist` from anywhere:

```bash
gitlist [path/to/directory]
```

If no directory is provided, `gitlist` will search the current directory.

## Example

```bash
$ gitlist /home/user/projects
/home/user/projects/project-a
/home/user/projects/another/project-b
```
