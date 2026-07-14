# 架构

本章面向希望理解 Lore Pages 内部原理或参与开发的读者。

## 模块结构

```
src/
├── main.rs                  # CLI 入口
├── lib.rs                   # Crate 根，重新导出四个子模块
├── parser.rs                # 默认解析器实现
├── render.rs                # 默认 HTML 渲染器实现
├── ir/
│   ├── mod.rs
│   ├── anchor.rs            # Anchor 枚举 — 文档节点
│   └── category.rs          # Category — 文档容器
└── framework/
    ├── mod.rs
    ├── parser.rs            # Parser trait
    ├── renderer.rs          # Renderer trait
    ├── converter.rs         # CategoryConverter — 解析+渲染管线
    ├── parser_config.rs     # ParserConfig — 解析配置
    ├── renderer_config.rs   # RenderConfig — 渲染配置
    └── category_config.rs   # CategoryConfig — 预留
```

## 数据流

```
           ┌─────────────┐
           │  .lore 文件  │
           └──────┬──────┘
                  │ &str
                  ▼
     ┌────────────────────────┐
     │  Parser::parse()       │
     │  - 逐行匹配语法         │
     │  - 生成 Anchor 节点列表  │
     └────────────┬───────────┘
                  │ Category
                  ▼
     ┌────────────────────────┐
     │  Category::auto_link_h2() │
     │  - 给 H2 标题添加自动链接  │
     └────────────┬───────────┘
                  │ Category (已修改)
                  ▼
     ┌────────────────────────┐
     │  Renderer::render()    │
     │  - 遍历 Anchor 列表     │
     │  - 生成 HTML 字符串      │
     └────────────┬───────────┘
                  │ String
                  ▼
           ┌─────────────┐
           │  .html 文件  │
           └─────────────┘
```

## IR: 中间表示

### Anchor 枚举

`Anchor` 是文档的最小单位，包含 13 种变体：

```rust
pub enum Anchor {
    Heading { level: u8, content: String, link: Option<String> },
    Paragraph { content: String },
    EmptyLine,
    BreakLine,
    PlaceHolderLine { content: String },
    UrlLink { name: String, url: String },
    LoreLink { name: String, path: String },
    Comment { content: String },
    Image { url: String },
    InnerUrlOpen { title: String, url: String },
    InnerUrlClose { title: String, url: String },
    InnerLoreOpen { title: String, path: String },
    InnerLoreClose { title: String, path: String },
}
```

### Category 结构体

```rust
pub struct Category {
    pub nodes: Vec<Anchor>,
}
```

`Category` 是对 `Anchor` 列表的简单包装，提供：

- `new()` — 创建空文档
- `push(anchor)` — 追加节点
- `auto_link_h2()` — 给每个 H2 标题设置自动链接

## Parser: 解析器

`LorePagesParser` 实现了 `Parser` trait：

```rust
pub trait Parser {
    fn parse(&self, input: &str, category_config: &CategoryConfig,
             parser_config: &ParserConfig) -> Category;
}
```

内部使用**匹配器注册表**模式：

```rust
type LineMatcher = fn(&str, &ParserConfig) -> Option<Anchor>;

fn matchers() -> &'static [LineMatcher] {
    &[
        match_heading,
        match_comment,
        match_image,
        // ... 共 11 个匹配器
    ]
}
```

每行依次尝试所有匹配器，首个成功匹配的返回 `Some(Anchor)`，全部失败则作为段落处理。

### 解析原语

`parser.rs` 还导出了多个 `pub` 解析函数，可以独立使用：

| 函数 | 用途 |
|------|------|
| `parse_heading` | 解析 `#` 标题 |
| `parse_comment` | 解析 `%` 注释 |
| `parse_image` | 解析 `\|` 图片 |
| `parse_placeholder` | 解析 `_` 占位符 |
| `parse_breakline` | 解析 `---` 分隔线 |
| `parse_url_link` | 解析 `名称 \| URL` |
| `parse_lore_link` | 解析 `名称 = 路径` |
| `parse_inner_url_open` | 解析 `- 标题 > URL` |
| `parse_inner_url_close` | 解析 `+ 标题 > URL` |
| `parse_inner_lore_open` | 解析 `- 标题 = 路径` |
| `parse_inner_lore_close` | 解析 `+ 标题 = 路径` |
| `parse_binary` | 通用中缀语法解析 |
| `parse_prefix` | 通用前缀语法解析 |
| `parse_domain` | 通用三段式语法解析 |
| `parse_repeated_prefix` | 通用重复前缀解析 |

## Renderer: 渲染器

`HtmlRenderer` 实现了 `Renderer` trait：

```rust
pub trait Renderer {
    fn render(&self, doc: &Category, category_config: &CategoryConfig,
              renderer_config: &RenderConfig, source_path: Option<&str>) -> String;
}
```

渲染过程：

1. 生成 `<!DOCTYPE html>` + `<html>` + `<head>` （含 meta、title、scripts）
2. 遍历 `doc.nodes`，每个节点调用 `render_node()` 转为 HTML 片段
3. 闭合 `</body>` + `</html>`

### 路径解析

渲染器内部包含几个路径处理函数：

- `resolve_link_from_source()` — 基于源文件所在目录修正相对路径
- `resolve_path()` — 应用 `link_base` 前缀
- `check_lore_link()` — 验证内部链接目标是否存在

### HTML 转义

`escape_html()` 函数对文本内容进行标准 HTML 转义：

| 字符 | 转义后 |
|------|--------|
| `&` | `&amp;` |
| `<` | `&lt;` |
| `>` | `&gt;` |
| `"` | `&quot;` |
| `'` | `&#39;` |

## Converter: 转换器

`CategoryConverter` 将 Parser 和 Renderer 组合为完整的转换管线：

```rust
pub struct CategoryConverter<'a, P, R> {
    parser: P,
    renderer: R,
    category_config: &'a CategoryConfig,
    renderer_config: &'a RenderConfig,
    parser_config: &'a ParserConfig,
}
```

提供三种转换方法：

- `convert_simple(raw)` — 使用存储的配置，无源文件信息
- `convert_with_source(raw, path)` — 同上，附加源文件路径（用于链接解析和警告）
- `convert(raw, cat_cfg, rend_cfg, pars_cfg, source)` — 完全自定义配置

## CLI 主流程

`main.rs` 的工作流：

1. 从 `Lore.toml` 加载 `RenderConfig` 和 `ParserConfig`（失败则用默认值）
2. 创建 `LorePagesParser` → `HtmlRenderer` → `CategoryConverter`
3. 尝试 `git ls-files *.lore` 快速收集源文件（失败则回退到目录递归遍历）
4. 逐文件转换，保持子目录结构
