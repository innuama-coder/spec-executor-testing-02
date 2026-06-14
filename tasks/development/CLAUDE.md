# CLAUDE.md - even-factorial

## 工作协议

你正在执行 `spec-executor-testing-02` 的 Rust 函数与测试开发任务。请先阅读 `docs/PRD.md`、`docs/HLD.md` 和 `docs/LLD.md`。

## 任务目标

创建 Rust binary crate，实现 `is_even` 与 `factorial`，并补齐内联测试。

## 交付要求

- 创建 `Cargo.toml`。
- 创建 `src/main.rs`。
- 实现 `pub fn is_even(n: i32) -> bool`。
- 实现 `pub fn factorial(n: u64) -> u64`，且 `factorial(0) == 1`。
- 添加覆盖偶数、奇数、`0!`、`1!`、`5!` 的测试。
- 不修改任务包和工作文档。

## 验证

运行 `cargo build` 和 `cargo test`，并在最终回复中报告结果。
