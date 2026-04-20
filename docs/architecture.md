# 项目架构概览

本文件概述本仓库的模块划分、数据流、扩展点与开发/发布建议，帮助新来贡献者快速理解项目结构与设计决策。

工作区（crates）职责

- `lang-core`：定义文档中间表示（IR），包括 `Category` 与 `Anchor`，任何解析器或渲染器都使用这些类型进行互操作。
- `lang-framework`：公共接口（`Parser`、`Renderer`）、配置加载（`Config`）以及 `Converter` 组合器（将解析器与渲染器组合成完整转换流程）。
- `lang-parser`：示例/默认的 Markdown 解析器（`MarkdownParser`），实现 `Parser` trait。
- `lang-impl`：HTML 渲染器实现（`HtmlRenderer`），实现 `Renderer` trait。
- `lang-cli`：命令行工具，负责加载 `Config`、组装 `Converter` 并在文件系统上执行批量转换。

数据流（高层）

1. `lang-cli` 读取源目录中的 `.lore` 文件（或单个文件）。
2. 使用 `Converter`（由 `lang-framework` 提供），调用 `Parser::parse` 将文本解析为 `lang-core::Category`。
3. 将 `Category` 传入 `Renderer::render`，得到输出字符串（例如完整 HTML 页面）。
4. CLI 将输出写入目标目录。

扩展点与约定

- Parser：实现 `lang_framework::Parser` 即可替换或新增解析器（可放在新的 crate 或 `lang-parser` 内）。
- Renderer：实现 `lang_framework::Renderer` 即可新增渲染目标（HTML、静态站点、JSON 等）。
- IR 扩展：若需新增节点类型（如列表、代码块、表格），优先在 `lang-core` 中定义新 `Anchor` 变体，并在解析器与渲染器中分别处理该变体。

示例：添加新节点（概要）

1. 在 `lang-core/src/ir.rs` 添加 `Anchor::CodeBlock { lang: Option<String>, code: String }`。
2. 在解析器（例如 `lang-parser`）中识别并生成该节点。
3. 在渲染器（例如 `lang-impl` 的 `render_node`）中增加对应的输出逻辑。

测试与示例

- 单元测试：每个 crate 内包含针对核心逻辑的单元测试（例如 `parse_heading`、`render`）。使用 `cargo test -p <crate>` 运行单个 crate 的测试。
- 集成/工作区测试：使用 `cargo test --workspace` 运行全部测试。
- 示例：`lang-impl/examples/render_basic.rs` 与 `lang-impl/examples/render_file.rs` 可作为运行时示例，使用 `cargo run -p lang-impl --example render_basic` 运行。

文档与发布

- 本仓库采用 `cargo doc --workspace` 生成 API 文档。已配置 GitHub Actions 将 `target/doc` 发布到 gh-pages（见 `.github/workflows/docs.yml`）。
- 变更记录请更新 `CHANGELOG.md`，发布新版本前运行全部测试并更新 crate 版本。

CI 与质量保证

- CI（见 `.github/workflows/ci.yml`）会在 push/PR 上运行：`cargo fmt -- --check`、`cargo clippy`（将警告视为错误）和 `cargo test`。
- 本地可用 `scripts/check.sh` 快速运行同样的检查。

开发流程建议

- 新功能优先写单元测试（或集成测试），确保 API 更改可被回退。
- 对 IR 的更改遵循最小侵入原则：尽量兼容现有节点，必要时在 `Converter` 层提供迁移/兼容策略。
- 文档示例应加到各 crate 的 `README.md` 中，并在 `docs/` 下补充架构说明与示例流程。

常见陷阱

- 不要在渲染器中直接依赖解析器实现；通过 `lang-framework` 的 trait 保持解耦。
- 当 IR 改动影响多个 crate（例如添加新 `Anchor`），请在同一个 PR 中更新 `lang-core`、解析器与渲染器，并添加测试。

文件参考

- 根 README: `README.md`，各 crate README: `lang-core/README.md`、`lang-framework/README.md` 等。
- CI: `.github/workflows/ci.yml`，Docs 发布: `.github/workflows/docs.yml`。
- 脚本: `scripts/check.sh`。

如需我把此文档转换为更详细的架构图或添加示例流程（示例输入/输出），我可以继续完善。
