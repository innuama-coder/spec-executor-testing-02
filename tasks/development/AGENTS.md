# AGENTS.md — even-factorial (codex executor)

> Loaded by spec-executor 2.0 when `executor: codex`. Copied to
> the worktree root during StartingExecutor. PROMPT.md is sent as
> the first user message via `send_input`.

## Task

Create a Rust binary crate from scratch. The repository has no Rust
code as baseline — produce `Cargo.toml` and `src/main.rs` implementing
two mathematical functions **with inline tests**:

- `pub fn is_even(n: i32) -> bool`
- `pub fn factorial(n: u64) -> u64`

## Constraints

- Create `Cargo.toml` and `src/main.rs` at the repo root.
- Standard library only. No external dependencies.
- Do not modify `tasks/development/`, `docs/`, `spec.yaml`,
  `README.md`, `.gitignore`.
- No `rust-toolchain.toml`.
- Functions and tests in `src/main.rs`; no `src/lib.rs`.

## Self-Verification (mandatory)

```
cargo build
cargo test
```

## Definition of Done

1. `Cargo.toml` exists.
2. `src/main.rs` exists with `is_even`, `factorial`, and a
   `#[cfg(test)] mod tests` containing `test_is_even` and
   `test_factorial`.
3. `cargo build` and `cargo test` both exit 0.
4. `tasks/development/` is byte-identical to the baseline.

## Out of Scope

- Refactoring, comments, integration tests.
- Adding CI, README, or `LICENSE` files.
- External crates or dependencies.
