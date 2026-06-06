# Delivery Standard — even-factorial

## Expected Work

Create a Rust binary crate from scratch implementing two mathematical
functions with inline `#[cfg(test)]` tests.

## Deliverable Files

| File | Condition | Verification |
|---|---|---|
| `Cargo.toml` | must exist | existence check |
| `src/main.rs` | must exist; `cargo test` passes | `cargo build && cargo test` |

## Expected Implementation

```
pub fn is_even(n: i32) -> bool;
pub fn factorial(n: u64) -> u64;
#[cfg(test)]
mod tests { test_is_even(){} test_factorial(){} }
```

## Example Passing State

```
$ cargo test
running 2 tests
test tests::test_is_even ... ok
test tests::test_factorial ... ok
test result: ok. 2 passed
```

## Task Package Integrity

`tasks/development/` and `docs/` must remain byte-identical to the
baseline.
