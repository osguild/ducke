# Run Rust Scripts

## Compile + run in one command
```
rustc -O myscript.rs && ./myscript
```

## Compile to a binary
```
rustc --crate-type bin -C opt-level=3 myscript.rs -o mybinary
```