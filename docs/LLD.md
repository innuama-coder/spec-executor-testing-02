# LLD - even-factorial

## 文件设计

| 文件 | 设计要求 |
| --- | --- |
| `Cargo.toml` | 包名建议为 `even-factorial`，版本为 `0.1.0`，edition 使用稳定 Rust edition。 |
| `src/main.rs` | 定义 `is_even`、`factorial`、`main` 和内联测试模块。 |

## 函数设计

| 函数 | 输入 | 输出 | 规则 |
| --- | --- | --- | --- |
| `is_even` | `i32` | `bool` | 当 `n % 2 == 0` 时返回 `true`。 |
| `factorial` | `u64` | `u64` | 返回从 1 到 `n` 的乘积；`n == 0` 时返回 1。 |

## 测试设计

| 测试 | 覆盖 |
| --- | --- |
| `test_is_even` | 一个偶数和一个奇数。 |
| `test_factorial` | `0!`、`1!`、`5!`。 |
