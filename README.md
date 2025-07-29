# rust-play

A personal collection of solved coding problems in idiomatic Rust.

This project is my commitment to solving at least one problem per day using Rust — not to memorise interview patterns, but to deeply understand and apply the language’s core features. Solutions prioritise clean, idiomatic Rust over traditional CS-heavy approaches.

For example, if a problem asks for the sum of elements in an array, the solution will be:

```rust
ar.iter().sum()
```

Not a hand-rolled loop. This repo is about mastering Rust, not reinventing data structures.

All problems solved are from [HackerRank](https://www.hackerrank.com/) unless stated otherwise.

---

## Structure

All problems are stored as individual `.rs` files under a flat structure. Filenames are based on the problem name:

```
problems/
├── sum.rs
├── sum_array.rs
└── ...
```

Each file:

* Contains a single solution function
* Includes a `pub fn run()` entry point for CLI execution
* Has unit tests using `#[cfg(test)]`

---

## Usage

Build and run with Cargo:

```sh
cargo run -- --problem sum_array
```

Or using the shorthand:

```sh
cargo run -- -p sum_array
```

This invokes the `run()` function for that specific problem.

### Run tests:

```sh
cargo test
```

---

## CLI Design

The CLI is powered by [`clap`](https://docs.rs/clap). It currently supports:

* `--problem` or `-p`: The name of the problem to run (matches the filename)

### Planned Features:

* `--inputs`: Provide comma-separated input directly to the CLI
* Measure time and space complexity of runs

---

## Philosophy

This repo isn’t about whiteboard heroics. It’s about:

* Writing idiomatic, expressive Rust
* Exploring the standard library deeply
* Building intuition through small, consistent reps
* Avoiding overengineering — no premature abstractions

---

## Contributing

This is a personal learning project, but contributions are welcome — especially around:

* Performance tuning
* Idiomatic improvements
* Runtime/memory profiling tools
* CLI polish

No leetcode-flexing, no verbosity, no framework bloat.

---

## License

MIT
