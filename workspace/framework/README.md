# lang-framework

`lang-framework` 提供了解析器与渲染器的公共接口、配置加载和 `Converter` 组合器。

主要组件

- `Parser`（trait）：将输入文本解析为 `lang_core::Category`。
- `Renderer`（trait）：将 `Category` 渲染为输出（例如 HTML）。
- `Config`：从 TOML 文件加载配置（路径、输出目录、css 地址）。
- `Converter`：组合 `Parser` 与 `Renderer`，提供一键转换接口。

如何实现自定义解析器/渲染器

示例：自定义简单解析器与渲染器，然后使用 `Converter`：

```rust
use lang_framework::{Converter, Parser, Renderer};
use lang_core::{Category, Anchor};

struct MyParser;
impl Parser for MyParser {
	fn parse(&self, input: &str) -> Category {
		let mut doc = Category::new();
		doc.push(Anchor::Paragraph { content: input.to_string() });
		doc
	}
}

struct MyRenderer;
impl Renderer for MyRenderer {
	fn render(&self, doc: &Category, title: &str, css_url: &str) -> String {
		format!("<html><head><title>{}</title></head><body>{:?}</body></html>", title, doc)
	}
}

let converter = Converter::new(MyParser, MyRenderer, "style.css".to_string());
let html = converter.convert("Hello world", "My Page");
```

配置示例：使用 `Config` 从 `Lore.toml` 加载路径与样式信息。

源码位置：`lang-framework/src/`
