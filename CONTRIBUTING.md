## Setup the Environment

You may need to setup some libraries in order to run this project locally.

- [macOS Setup](#macOS-Setup)

### macOS Setup

- SDL2: Install it with brew using `brew install SDL2`

Then either extend the `LIBRARY_PATH` environment variable to include
Homebrew's installed libraries by adding:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

To your `~/.zshenv` or `~/.bash_profile` or specify the environment variable
when running `cargo run`.

```bash
LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib" cargo run roms/INVADERS
```

> Alternative option could be: `LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib" cargo run -- ./roms/INVADERS`
