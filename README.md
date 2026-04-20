# Lore Pages

轻量级的多 crate Rust 项目，用于将简单的 Lore/Markdown 文档转换为 HTML。

核心组件：

- `lang-core`：定义文档的中间表示（IR），包含 `Document` 与 `Node`。
- `lang-framework`：提供解析器/渲染器的 trait、配置加载与 `Converter` 组合器。
- `lang-parser`：简化的 Markdown 解析器（支持 1-4 级标题与段落）。
- `lang-impl`：HTML 渲染器实现（`HtmlRenderer`）。
- `lang-cli`：命令行工具，组合解析器与渲染器以批量转换目录下的 `.lore` 文件。

快速开始
---------

1. 构建项目：

```bash
cargo build --release
```

2. 使用命令行工具进行转换（默认读取仓库根目录下的 `Lore.toml` 配置）：

```bash
cargo run -p lang-cli --release
```

3. 运行全部测试：

```bash
cargo test --all
```

示例配置（Lore.toml）
--------------------

在仓库根目录放置 `Lore.toml`，例如：

```toml
from_lore_path = "./lore"
to_html_path = "./html"
css_url = "style.css"
```

示例：作为库使用
-----------------

下面示例展示如何在项目中组合解析器与渲染器：

```rust
use lang_core::{Document, Node};
use lang_parser::MarkdownParser;
use lang_impl::HtmlRenderer;
use lang_framework::Converter;

let parser = MarkdownParser;
let renderer = HtmlRenderer;
let converter = Converter::new(parser, renderer, "style.css".to_string());

let html = converter.convert("# 标题\n第一段", "示例页面");
println!("{}", html);
```

运行示例
--------

项目中包含若干示例：

```bash
cargo run -p lang-impl --example render_basic
cargo run -p lang-impl --example render_file
```

项目结构速览
-------------

- Cargo.toml（工作区）
- lang-core/
- lang-framework/
- lang-parser/
- lang-impl/
- lang-cli/

贡献与联系
-----------

欢迎提交 issue / PR。若需我继续：

- 为 README 增加更多使用示例（例如 CI、Docker）
- 增补 API 文档或示例页面模板

许可
---

详见仓库根目录的 [LICENSE](LICENSE)。

Badges（占位）
----------------

下面是常用 badge 的示例 Markdown 语法：

- CI 状态（Actions `ci.yml`）：

	[![CI](https://github.com/dotCorain/lore-pages/actions/workflows/ci.yml/badge.svg)](https://github.com/dotCorain/lore-pages/actions/workflows/ci.yml)

- Docs 发布（Actions `docs.yml`）：

	[![Docs](https://github.com/dotCorain/lore-pages/actions/workflows/docs.yml/badge.svg)](https://github.com/dotCorain/lore-pages/actions/workflows/docs.yml)

- Coverage（Actions `coverage.yml`）：

	[![Coverage](https://github.com/dotCorain/lore-pages/actions/workflows/coverage.yml/badge.svg)](https://github.com/dotCorain/lore-pages/actions/workflows/coverage.yml)

测试覆盖说明
---------------

已在 `.github/workflows/coverage.yml` 中添加了一个运行 `cargo-tarpaulin` 的工作流，它会在 push/PR 时生成覆盖报告并作为 artifact 上传。若要将覆盖结果可视化为 badge（例如 Codecov），可在仓库中配置 Codecov/GitHub App 并在 workflow 中上传到对应服务，然后在 README 中替换上方占位链接。

