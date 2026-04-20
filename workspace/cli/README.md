# lang-cli

`lang-cli` 是命令行工具，负责：

- 加载 `Lore.toml` 配置（通过 `lang-framework::Config`）。
- 遍历源目录（`from_lore_path`），将 `.lore` 文件解析并渲染为 HTML，输出到 `to_html_path`。

快速使用

```bash
# 在仓库根目录运行（默认使用 Lore.toml）
cargo run -p lang-cli --release
```

示例配置（Lore.toml）

```toml
from_lore_path = "./lore"
to_html_path = "./html"
css_url = "style.css"
```

示例：如何为单个文件运行并查看输出

```bash
cargo run -p lang-cli --release -- /path/to/single/file.lore
# 或者直接运行示例二进制以查看效果（根据本地构建）
cargo run -p lang-impl --example render_basic
```

注意

- CLI 负责组装 `Converter` 并递归处理目录；具体的解析器与渲染器由库实现提供（例如 `lang-parser`、`lang-impl`）。
- 标题提取逻辑（`extract_title`）默认使用文件名作为页面标题；如需更复杂的元数据，请在预处理阶段添加 Front Matter 解析。

源码位置：`lang-cli/src/main.rs`
