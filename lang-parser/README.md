# lang-parser

`lang-parser` 实现了一个简化的 Markdown 解析器（`MarkdownParser`），当前支持：

- 标题：`#`、`##`、`###`、`####`（要求 `#` 后面有空格）
- 段落：单行文本解析为 `Paragraph` 节点（当前实现会把空行当作空段落）

导出函数

- `parse_heading(line: &str) -> Option<(u8, String)>`：检测并解析标题行（已在 crate 根导出）。

行为注意

- 空行会被视为一个空的 `Paragraph` 节点（当前实现）；因此文档中的空行会变成空段落。
- 标题解析仅支持最多 4 级标题；没有空格或超过 4 个 `#` 会被视为普通文本。

使用示例

```rust
use lang_parser::MarkdownParser;
use lang_framework::Parser;

let parser = MarkdownParser;
let doc = parser.parse("# 标题\n第一段文本");
```

测试与示例

- 仓库内包含多组单元测试覆盖 `parse_heading` 与解析器整体行为，运行 `cargo test -p lang-parser`。

源码位置：`lang-parser/src/parser.rs`
