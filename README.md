# pong

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.

# Incorrect Documentation
This project was made with the guidance of the Amethyst Documentation. Unfortunately, it was very out of date at the time this project was created, and their instructions produced large amounts of errors that prevented compilation. These are several notes about what was incorrect:
* `DjSystem` should be `DjSystemDesc` and needs to be included with `with_system_desc()` instead of `with()`. This will create a missing resource error.
* Several Unused imports
* In `winner.rs`, there's instances where `text` needs to be replaced with `ui_text`
* An entire step of updating the `winner.rs` is excluded. This will create a missing resource error.
