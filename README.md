A terminal snake game written in Rust with the use of pancurses.
It's supposed to be equal feature-wise to my [C++ snake](https://github.com/GOKOP/snake) but isn't yet.

Stuff that the other snake currently does and this one doesn't:
- config through command line options

Stuff that I want to make but neither snake has it yet:
- config through an in-game menu
- p2p multiplayer (yeah I know multiplayer snake is a great idea)

## Compiling
Make sure you have Rust toolchain installed and run `cargo build --release`.

### Windows
This program uses pancurses Rust library which uses pdcurses C library in the backend on Windows.
Pdcurses can run either in a special "terminal" window which has better support for unix-like colors and some other stuff or in native Windows console.
The first option is the default. If you wish to compile it using native Windows cmd, add "win32" to pancurses features in Cargo.toml:
```
features = ["wide", "win32"]
```
