# PROMPT.md

## 任务目标

开始执行 `even-factorial` 任务。在空仓库中创建 Rust binary crate，实现 `is_even` 与 `factorial` 两个公共函数，并补齐内联单元测试。

## 必读上下文

1. `docs/PRD.md`
2. `docs/HLD.md`
3. `docs/LLD.md`
4. `CLAUDE.md` 或 `AGENTS.md`

## 交付物

- `Cargo.toml`
- `src/main.rs`

## 验收标准

- `cargo build` 成功。
- `cargo test` 成功。
- 测试覆盖偶数、奇数、`0!`、`1!` 和 `5!`。
- 不修改 `tasks/development/`、`docs/`、`README.md`、`.gitignore` 或 `spec.yaml`。

## Handoff

最终回复包含修改摘要、验证命令和验证结果。
