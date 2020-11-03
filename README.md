# `cdwin`

Simple program to allow typing `cdwin <some windows path>` in a WSL terminal to navigate to the corresponding directory.

No pretty code, no good installation instructions, no real error handling; use at your own risk.

## Installation Instructions

(assuming a bash shell and no particular considerations of where the program ends up)

Clone the repository and compile&install the executable using `cargo install --path .`.

Add `source <path to cdwin.sh>` to your `~/.bashrc` file.

Restart your terminal or execute `source ~/.bashrc` and `cdwin` should be available!
