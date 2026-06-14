# spec-executor-testing-02 - even-factorial

Rust 函数与测试开发型独立测试仓库。该仓库用于验证 spec-executor 是否能够驱动 Claude/Codex 创建 Rust crate、实现基础函数并补齐内联单元测试。

## 目录

- `docs/PRD.md`：产品需求。
- `docs/HLD.md`：高层设计。
- `docs/LLD.md`：详细设计。
- `docs/DELIVERY.md`：交付说明模板。
- `tasks/development/`：spec-executor task package。

## 运行

```bash
spec-executor run --spec tasks/development/spec.yaml --workspace ./workspace
```
