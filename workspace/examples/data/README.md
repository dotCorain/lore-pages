示例数据目录
=================

此目录包含用于本项目的最小示例输入文件，便于本地快速验证解析器与渲染器行为。

文件列表：

- `sample1.lore`：简单标题 + 段落示例。
- `sample2.lore`：包含多个标题的更长示例。

使用方法：

1. 在项目根目录运行 CLI：

```bash
cargo run -p lang-cli --example convert -- ./examples/data ./out
```

（注：示例命令需根据你的 `lang-cli` 示例实现调整；主要目的是展示如何引用示例数据。）
