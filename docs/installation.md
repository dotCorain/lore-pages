# 安装

## 从源码编译

确保已安装 Rust 工具链（[rustup.rs](https://rustup.rs)）：

```bash
git clone https://github.com/dotCorain/lore-pages.git
cd lore-pages
cargo build --release
```

编译产物在 `target/release/lore-pages`。

## 使用 Makefile

项目提供了 Makefile 简化操作：

```bash
# 当前平台构建
make build

# 构建所有平台 (Linux / macOS ARM / macOS Intel / Windows)
make build-all

# 安装到 /usr/local/bin (macOS / Linux)
make install

# 运行测试
make test

# 格式化 + Lint
make fmt
make lint
```

## 验证安装

```bash
lore-pages --help
```

如果能正常显示帮助信息，说明安装成功。
