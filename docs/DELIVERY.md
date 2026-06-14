# DELIVERY - even-factorial

## 验收用途

本文档用于人工复核 `even-factorial` 任务是否完成。执行者无需修改本文档，但最终回复必须提供与本文档一致的验收证据。

## 交付物

| 交付物 | 验收要点 |
| --- | --- |
| `Cargo.toml` | 位于仓库根目录，声明 Rust binary crate。 |
| `src/main.rs` | 包含 `is_even`、`factorial` 和内联单元测试。 |

## 验收命令

```bash
cargo build
cargo test
```

## 通过标准

- `cargo build` 退出码为 0。
- `cargo test` 退出码为 0。
- 测试覆盖偶数、奇数、`0!`、`1!` 和 `5!`。
- `tasks/development/`、`docs/`、`README.md`、`.gitignore` 和 `spec.yaml` 未被修改。
