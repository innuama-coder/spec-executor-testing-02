# CLAUDE.md — even-factorial (claude executor)

> Loaded by spec-executor 2.0. Copied from `tasks/development/CLAUDE.md`
> to the worktree root during StartingExecutor. PROMPT.md is sent
> as the first user message via `send_input`.

## Mission

Create a Rust binary crate from scratch. The repository has no Rust
code as baseline — you must produce `Cargo.toml` and `src/main.rs`
implementing two public functions **with inline tests**, so that
`cargo build && cargo test` passes:

- `pub fn is_even(n: i32) -> bool` — returns `true` if `n` is even
- `pub fn factorial(n: u64) -> u64` — returns `n!`; `0!` returns `1`

## Working Agreement

- **Create** `Cargo.toml` and `src/main.rs` at the repo root.
- **Standard library only.** No external dependencies.
- **Do not modify** `tasks/development/`, `docs/`, `spec.yaml`,
  `README.md`, `.gitignore`, or any file not related to the crate
  you are creating.
- **No `rust-toolchain.toml`.** Build with the resolved stable
  toolchain.
- **Functions and tests in `src/main.rs`.** Do not create
  `src/lib.rs` or a separate library crate.

Tests must cover at least:
- `is_even` with even and odd inputs
- `factorial` with `0`, `1`, and a small value (e.g. `5 → 120`)

The `main` function must call both functions and print results so
the crate compiles; the exact output is not verified by the spec.

## Self-Verification

```
cargo build
cargo test
```

Both must exit 0; all tests must pass.

## Definition of Done

1. `Cargo.toml` exists and defines a binary crate.
2. `src/main.rs` exists with `is_even`, `factorial`, and a
   `#[cfg(test)] mod tests` containing `test_is_even` and
   `test_factorial`.
3. `cargo build` and `cargo test` both exit 0.
4. `tasks/development/` is byte-identical to the baseline
   (verified by the spec).

## Out of Scope

- External crates or dependencies.
- Integration tests, benchmarks, or examples.
- Adding a `rust-toolchain.toml`.
- Modifying any file outside the crate you create.
- `Cargo.lock`.
