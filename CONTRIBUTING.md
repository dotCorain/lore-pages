# 贡献指南

感谢你对本项目的关注与贡献！本文件说明如何在本仓库中构建、测试、规范化代码，以及提交 PR 的基本流程。

开发环境准备
----------------

推荐使用最新稳定版 Rust 工具链：

```bash
rustup toolchain install stable
rustup component add rustfmt clippy
```

本地常用命令
----------------

```bash
# 构建（workspace）
cargo build

# 运行所有测试
cargo test --workspace

# 检查代码格式（CI 也会检查）
cargo fmt -- --check

# 静态检查（启用为错误）
cargo clippy --workspace --all-targets -- -D warnings

# 运行示例（按需替换 crate 和 example 名称）
cargo run -p lang-impl --example render_basic
```

代码风格与检查
-------------------

- 请在提交前运行 `cargo fmt` 和 `cargo clippy`。CI 会在 push/PR 时自动检查并阻止不合格的提交。
- 我们采用 Rust 社区推荐的格式与 lint 规则；如需局部放行，请在代码中使用合适的 `allow` 注解并在 PR 中说明理由。

提交规范与 PR 流程
---------------------

- 分支命名建议：`feat/...`、`fix/...`、`docs/...`、`chore/...`。
- 提交信息建议遵循 Conventional Commits（例如 `feat(parser): 支持多级标题`）。
- 每个 PR 保持小而专注：说明变更目的、相关 issue、变更范围、是否需要迁移说明。
- PR 检查清单：
  - 代码编译通过且测试覆盖相关改动
  - 本地运行 `cargo fmt`、`cargo clippy` 无错误
  - 更新或新增对应 README/文档（如适用）

问题报告与讨论
-----------------

- 欢迎通过 GitHub Issues 报告 bug 或讨论特性需求。
- 请提供可复现的最小示例（输入文件、复现步骤、期望输出）。

发布与变更日志
-----------------

- 在发布新版本前，请更新 `CHANGELOG.md`（如果存在），并在 PR 中注明版本影响。

其他说明
--------

- 贡献者应尽量为重大变更添加单元测试或集成测试。
- 如需帮助，我可以代为添加 CI、模板、或示例脚本。欢迎告诉我下一步要我做什么。
