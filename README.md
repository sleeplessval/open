# open

This is a Rust rewrite of my `open` script from my shell scripts. It's written to be faster and more customizable; it now features "local" configs and a zero-operand command, allowing the user to specify how files should be opened differently, and for opening a project, etc.

For example, for

```ini
[open]
# zero-operand command
command = atom .

[.md]
command = typora

[.rs]
command = atom

[filename:.gitignore]
command = vim
shell = true
```

I can use `open` to open the directory in Atom, or I could use `open src/main.rs` to open `main.rs` in Atom, and I can specify these on a per-project basis.

For directories with a local config, any missing values will be filled in by the global config (`~/.config/open.conf`), which means local configs can be shorter.

