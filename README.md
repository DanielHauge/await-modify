# await-modify

This repository contains a very small program that waits for a modification made to a file given as input.

This was made as a fun/learning project and as a result of lack of inotify-tools on windows.

## Installation

Clone repo and build with cargo.

The program can then be put in path or alias can be made.

```sh
cargo build -r # Builds release mode
# Alias can be put in bash profile
alias await-modify='path/to/repo/target/release/await-modify' # including .exe if windows
```

## Usage

The program will await a modification made to the input file.

```sh
await-modify <file>
```

Tip: it can be used to automate builds, tests or other things on file saves. Like compiling latex, markdown, unit tests etc.

```sh
# For example, in an exported function, or even hard-coded in a script file.
while true; do
    await-modify "$file"
    buildtex "$file" # or build code project, unit tests, etc.
done
```
