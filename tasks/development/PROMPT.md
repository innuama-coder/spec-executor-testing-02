# PROMPT.md — even-factorial (universal first instruction)

> Sent as the first user message to whichever executor is launched,
> via `send_input`.

---

Begin the **even-factorial** task.

This repository has no Rust code. Create a binary crate from scratch
with two functions and inline tests, so that `cargo test` passes.

Read your working agreement: `CLAUDE.md` (claude) or `AGENTS.md`
(codex) at the worktree root.

Steps:
1. Create `Cargo.toml` and `src/main.rs`.
2. Implement `pub fn is_even(n: i32) -> bool` and
   `pub fn factorial(n: u64) -> u64` (with `factorial(0) == 1`).
3. Add a `#[cfg(test)] mod tests` with `test_is_even` (even+odd) and
   `test_factorial` (0!, 1!, 5!).
4. Run `cargo build` and `cargo test`.
5. Confirm all tests pass, then stop.

Constraints (full list in your agreement file):
- Standard library only. No external dependencies.
- Do not touch `tasks/development/`, `docs/`, `spec.yaml`,
  `README.md`, `.gitignore`.
- Do not add a `rust-toolchain.toml`.
