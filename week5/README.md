# Week 5 - 2D Point Mapping

A Rust project demonstrating struct input/output with horizontal and vertical point flipping.

## Features

- `InputPoint` struct with original position and flip direction
- `OutputPoint` struct with original and output positions
- Horizontal flip: negates X coordinate
- Vertical flip: negates Y coordinate

## Project Structure

```
src/
├── main.rs           # Entry point with usage examples
└── point/
    ├── mod.rs        # Module declaration and re-exports
    └── point.rs      # Point structs and flip logic
```

## Running

```bash
cargo run
```

## Key Concepts Learned

- Module organization with `mod.rs`
- Public/private visibility (`pub`)
- Enum with `Copy` and `Clone` traits
- `fmt::Display` trait for custom formatting
- Re-exports with `pub use`