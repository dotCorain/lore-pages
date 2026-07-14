# 扩展

Lore Pages 基于 trait 架构，你可以替换解析器或渲染器来扩展功能。

## 自定义解析器

实现 `Parser` trait：

```rust
use lore_pages::framework::parser::Parser;
use lore_pages::framework::category_config::CategoryConfig;
use lore_pages::framework::parser_config::ParserConfig;
use lore_pages::ir::category::Category;
use lore_pages::ir::anchor::Anchor;

struct MyParser;

impl Parser for MyParser {
    fn parse(
        &self,
        input: &str,
        _category_config: &CategoryConfig,
        _parser_config: &ParserConfig,
    ) -> Category {
        let mut doc = Category::new();

        for line in input.lines() {
            if line.is_empty() {
                doc.push(Anchor::EmptyLine);
            } else {
                doc.push(Anchor::Paragraph {
                    content: format!("[MyParser] {}", line),
                });
            }
        }

        doc
    }
}
```

使用时替换 `LorePagesParser`：

```rust
use lore_pages::framework::converter::CategoryConverter;

let converter = CategoryConverter::from_config(
    MyParser,                  // 用你自己的解析器
    HtmlRenderer,
    &CategoryConfig::default(),
    &RenderConfig::default(),
    &ParserConfig::default(),
);
```

## 自定义渲染器

实现 `Renderer` trait：

```rust
use lore_pages::framework::renderer::Renderer;
use lore_pages::framework::category_config::CategoryConfig;
use lore_pages::framework::renderer_config::RenderConfig;
use lore_pages::ir::category::Category;
use lore_pages::ir::anchor::Anchor;

struct JsonRenderer;

impl Renderer for JsonRenderer {
    fn render(
        &self,
        doc: &Category,
        _category_config: &CategoryConfig,
        _renderer_config: &RenderConfig,
        _source_path: Option<&str>,
    ) -> String {
        let mut json = String::from("[\n");
        for (i, node) in doc.nodes.iter().enumerate() {
            let item = match node {
                Anchor::Heading { level, content, .. } => {
                    format!(r#"  {{"type": "heading", "level": {}, "content": "{}"}}"#, level, content)
                }
                Anchor::Paragraph { content } => {
                    format!(r#"  {{"type": "paragraph", "content": "{}"}}"#, content)
                }
                _ => format!(r#"  {{"type": "other"}}"#),
            };
            json.push_str(&item);
            if i + 1 < doc.nodes.len() {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("]\n");
        json
    }
}
```

## 组合自定义 Pipeline

用 `CategoryConverter` 自由组合 Parser 和 Renderer：

```
┌──────────┐     ┌──────────┐     ┌────────────┐
│ 你的 Parser │ ──→ │ Category │ ──→ │ 你的 Renderer │
└──────────┘     └──────────┘     └────────────┘
```

例如，你可以：

- 用 `LorePagesParser` 解析 `.lore`，然后用自定义渲染器输出 JSON
- 用自定义解析器读取 CSV 文件，然后用 `HtmlRenderer` 生成 HTML 表格
- 在 Parser 和 Renderer 之间插入一个 `Category` 变换步骤（如过滤节点、排序等）

## 添加新的语法元素

如果你想给 Lore 语法增加新的元素，只需三步：

### 步骤 1: 在 `Anchor` 枚举中添加变体

```rust
// src/ir/anchor.rs
pub enum Anchor {
    // ... 现有变体 ...
    /// 自定义的高亮引用块
    Blockquote {
        content: String,
    },
}
```

### 步骤 2: 写解析适配器

```rust
// src/parser.rs

// 解析函数
fn parse_blockquote(line: &str) -> Option<String> {
    line.strip_prefix("> ").map(|s| s.to_string())
}

// 匹配器适配器
fn match_blockquote(line: &str, _: &ParserConfig) -> Option<Anchor> {
    parse_blockquote(line).map(|content| Anchor::Blockquote { content })
}

// 注册到匹配器列表
fn matchers() -> &'static [LineMatcher] {
    &[
        match_heading,
        match_blockquote,  // ← 新增
        match_comment,
        // ...
    ]
}
```

### 步骤 3: 在渲染器中添加分支

```rust
// src/render.rs 的 render_node() 函数
match node {
    // ... 现有分支 ...
    Anchor::Blockquote { content } => {
        format!("  <blockquote>{}</blockquote>", escape_html(content))
    }
}
```

三步完成。所有现有测试仍然通过（你当然也应该为新语法添加测试）。

## 添加新的配置项

### ParserConfig

1. 在 `ParserTable` struct 中添加 `Option<String>` 字段
2. 在 `ParserConfig` struct 中添加 `String` 字段
3. 在 `Default` 实现中设默认值
4. 在 `from_file()` 中添加 unwrap_or 映射

### RenderConfig

类似的流程：

1. 在 `RendererTable` 中添加 `Option<T>` 字段
2. 在 `RenderConfig` 中添加 `T` 字段
3. 在 `Default` 中设默认值
4. 在 `from_file()` 中添加映射
