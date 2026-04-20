# lang-impl

`lang-impl` 提供 `HtmlRenderer`，用于将 `lang_core::Document` 渲染为完整的 HTML 页面。

行为说明

- `Node::Heading` 渲染为对应的 `<hN>` 标签。
- `Node::Paragraph` 渲染为 `<p>`；空段落渲染为 `<br>`。
- 生成基本文档头（`<head>`、`<title>`、`<link rel=stylesheet>` 等）。

安全说明

渲染器内部对文本进行了基本的 HTML 转义（例如 `&`, `<`, `>` 等），以降低注入风险。请注意这不是全面的 XSS 防护策略：如果渲染来自不受信任的来源，请在应用层根据需要增加更严格的过滤或沙箱策略。

快速示例

```rust
use lang_core::{Document, Node};
use lang_impl::HtmlRenderer;
use lang_framework::Renderer;

let mut doc = Document::new();
doc.push(Node::Heading { level: 1, content: "标题".to_string() });
let renderer = HtmlRenderer;
let html = renderer.render(&doc, "页面标题", "style.css");
println!("{}", html);
```

源码位置：`lang-impl/src/renderer.rs`
