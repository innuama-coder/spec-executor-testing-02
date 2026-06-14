# PRD - even-factorial

## 背景

spec-executor 需要一个带测试的 Rust 小型开发任务，用于验证执行器是否能够完成函数实现、测试编写和本地验证闭环。

## 目标

创建 Rust binary crate，实现 `is_even` 与 `factorial` 两个公共函数，并提供内联单元测试。

## 功能需求

| ID | 需求 |
| --- | --- |
| FR-001 | 实现 `pub fn is_even(n: i32) -> bool`。 |
| FR-002 | 实现 `pub fn factorial(n: u64) -> u64`，其中 `factorial(0) == 1`。 |
| FR-003 | 添加 `#[cfg(test)] mod tests`，覆盖偶数、奇数、`0!`、`1!` 和 `5!`。 |
| FR-004 | 仅使用 Rust 标准库。 |

## 约束

不得修改 `tasks/development/`、`docs/`、`README.md`、`.gitignore` 或 `spec.yaml`。

## 验收

`cargo build` 与 `cargo test` 必须通过。
