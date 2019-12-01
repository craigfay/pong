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
* `DjSystem` should be `DjSystemDesc` and needs to be included with `with_system_desc()` instead of `with()`
* Several Unused imports
* In `winner.rs`, there's instances where `text` needs to be replaced with `ui_text`