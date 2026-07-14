# Lore Pages

轻量级静态网站生成器，将自定义 `.lore` 标记语言编译为 HTML。

[![License]](#)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)

## 理念

- **简洁**：一行文本就是段落，`#` 开头就是标题。几分钟上手。
- **可配置**：所有标记符都可以在 `Lore.toml` 中自定义。
- **可扩展**：基于 trait 架构，支持替换解析器和渲染器。

## 快速开始

### 安装

```bash
git clone https://github.com/dotCorain/lore-pages.git
cd lore-pages
cargo build --release
```

### 创建项目

```
my-site/
├── Lore.toml
└── docs-src/
    ├── index.lore
    └── notes/
        └── rust.lore
```

**Lore.toml**

```toml
[renderer]
site_title = "我的知识库"
from_lore_path = "./docs-src"
to_html_path = "./docs"
lang = "zh-CN"
```

**docs-src/index.lore**

```
# 我的知识库

## 编程

Rust 是一门系统编程语言。

官网 | https://www.rust-lang.org
笔记 = rust

## 数学

微积分笔记整理中...
```

### 编译

```bash
lore-pages
```

输出到 `docs/` 目录，可直接部署。

## 语法速览

| 语法 | 写法 | 输出 |
|------|------|------|
| 标题 | `## 小节` | `<h2>` |
| 段落 | 普通文本行 | `<p>` |
| 外部链接 | `名称 \| URL` | `<a href="URL">` |
| 内部链接 | `名称 = 路径` | `<a href="路径.html">` |
| 注释 | `% 内容` | `<!-- 内容 -->` |
| 分隔线 | `---` | `<br>` |
| 图片 | `\| URL` | `<img>` |
| 折叠块 | `- 标题 > URL` | 可展开区域 |

H2 标题自动生成指向子页面 (`标题/index`) 的链接。

## 配置

所有标记符可通过 `Lore.toml` 的 `[parser]` 段自定义：

```toml
[parser]
url_link_key = "->"         # 默认 |
comment_key = "//"          # 默认 %
breakline_key = "***"       # 默认 ---

[renderer]
site_title = "站点名称"
from_lore_path = "./docs-src"
to_html_path = "./docs"
lang = "zh-CN"
link_base = ""              # 子目录部署时设置
scripts = []                # 注入的 JS 文件
```

详见 [配置文档](docs/configuration.md)。

## 架构

```
.lore 源文件
    │
    ▼
┌─────────┐     ┌──────────┐     ┌──────────┐
│ Parser  │ ──→ │ Category │ ──→ │ Renderer │
└─────────┘     └──────────┘     └──────────┘
                                      │
                                      ▼
                                  .html 文件
```

- **`src/ir/`** — 中间表示：`Anchor`（13 种节点类型）和 `Category`（文档容器）
- **`src/parser.rs`** — 默认解析器，基于匹配器注册表模式
- **`src/render.rs`** — 默认 HTML 渲染器
- **`src/framework/`** — `Parser`/`Renderer` trait、配置加载、`CategoryConverter` 管线

详细架构说明见 [架构文档](docs/architecture.md)。

## 扩展

Lore Pages 的 trait 架构让你可以自由替换解析器或渲染器：

```rust
let converter = CategoryConverter::from_config(
    MyParser,        // 自定义解析器
    HtmlRenderer,    // 默认渲染器
    &cat_cfg, &rend_cfg, &pars_cfg,
);
```

添加新语法只需三步：定义 `Anchor` 变体 → 注册匹配器 → 添加渲染分支。详见 [扩展指南](docs/extending.md)。

## 文档

完整文档见 [`docs/`](docs/SUMMARY.md)：

- [介绍](docs/introduction.md)
- [安装](docs/installation.md)
- [配置参考](docs/configuration.md)
- [语法参考](docs/syntax.md)
- [架构说明](docs/architecture.md)
- [扩展指南](docs/extending.md)

## 开发

```bash
# 构建
cargo build --release

# 测试（97 个测试）
cargo test

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# 格式化
cargo fmt --all
```

## 许可

MIT
