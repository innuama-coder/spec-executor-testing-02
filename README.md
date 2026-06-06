# spec-executor-testing-02 — even-factorial

spec-executor 2.0 测试用例：从零构建 Rust binary crate，实现 `is_even` 和 `factorial` 两个函数并附带 `#[cfg(test)]` 测试。

## 目录

- `tasks/development/spec.yaml` — spec-executor 2.0 入口
- `tasks/development/CLAUDE.md` / `AGENTS.md` / `PROMPT.md` — 任务包
- `docs/DELIVERY.md` — 验收标准说明

## 运行

```
spec-executor run --spec tasks/development/spec.yaml --workspace ./workspace
```
