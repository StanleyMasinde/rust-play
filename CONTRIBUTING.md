# Contributing

Thanks for your interest in contributing to `rust-play`! This project is primarily a personal learning tool, but contributions are welcome — especially around performance, idiomatic improvements, and CLI tooling.

---

## Problem Solutions

To add a new solution:

1. Create a new file under `src/solutions/` named after the problem (e.g., `sum_array.rs`).
2. In that file:

   * Define a function named exactly as the file (e.g., `fn sum_array(...) -> ... {}`).
   * Add a `pub fn run()` function that calls this named function.
   * Add unit tests under `#[cfg(test)]` matching the HackerRank sample inputs and outputs.
3. Open `src/runner/mod.rs` and add your problem to the dispatcher:

```rust
map.insert("sum_array".to_string(), solutions::sum_array::run);
```

Add this just above the `map` return statement inside `get_problem_hashmap()`.

---

## Guidelines

* File and function names must match.
* All submissions must include tests that mirror HackerRank test cases.
* Only the Rust standard library is allowed — no external crates beyond those already in the project.
* `run()` should remain simple and not expect dynamic inputs for now.

---

## General Improvements

Contributions beyond problem solutions are welcome. This includes:

* CLI polish or features
* Runtime/memory benchmarking tools
* Refactoring
* Readability/idiomatic suggestions

Open a PR with a clear description of your change.

---

Thanks for helping improve `rust-play`!
