# 版本信息
VERSION := $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')
BIN_NAME := lore-pages

# 目标平台
LINUX_TARGET := x86_64-unknown-linux-musl
MACOS_TARGET := aarch64-apple-darwin
MACOS_X86_TARGET := x86_64-apple-darwin
WINDOWS_TARGET := x86_64-pc-windows-gnu

# 输出目录
RELEASE_DIR := target/release
LINUX_DIR := target/$(LINUX_TARGET)/release
MACOS_DIR := target/$(MACOS_TARGET)/release
WINDOWS_DIR := target/$(WINDOWS_TARGET)/release

# 安装目录
INSTALL_DIR := /usr/local/bin

.PHONY: all build build-linux build-macos build-windows \
        install clean release test fmt lint help

all: build

# 默认构建（当前平台）
build:
	cargo build --release

# Linux 交叉编译
build-linux:
	@echo "Adding Linux target..."
	rustup target add $(LINUX_TARGET)
	@echo "Building for Linux..."
	cargo build --release --target $(LINUX_TARGET)

# macOS ARM (M1/M2/M3)
build-macos:
	@echo "Adding macOS ARM target..."
	rustup target add $(MACOS_TARGET)
	@echo "Building for macOS ARM..."
	cargo build --release --target $(MACOS_TARGET)

# macOS Intel
build-macos-intel:
	@echo "Adding macOS Intel target..."
	rustup target add $(MACOS_X86_TARGET)
	@echo "Building for macOS Intel..."
	cargo build --release --target $(MACOS_X86_TARGET)

# Windows
build-windows:
	@echo "Adding Windows target..."
	rustup target add $(WINDOWS_TARGET)
	@echo "Building for Windows..."
	cargo build --release --target $(WINDOWS_TARGET)

# 构建所有平台
build-all: build-linux build-macos build-macos-intel build-windows

# 安装到系统（当前平台）
install: build
	sudo cp $(RELEASE_DIR)/$(BIN_NAME) $(INSTALL_DIR)/

# 本地运行
run: build
	./$(RELEASE_DIR)/$(BIN_NAME)

# 清理
clean:
	cargo clean
	rm -rf docs-target

# 发布到 GitHub Release
release: build-all
	@echo "Creating release v$(VERSION)..."
	@echo "Files to upload:"
	@ls -lh $(LINUX_DIR)/$(BIN_NAME)
	@ls -lh $(MACOS_DIR)/$(BIN_NAME)
	@ls -lh $(WINDOWS_DIR)/$(BIN_NAME).exe
	@echo ""
	@echo "Run this to create release:"
	@echo "gh release create v$(VERSION) \\"
	@echo "  $(LINUX_DIR)/$(BIN_NAME)#$(BIN_NAME)-linux-x86_64 \\"
	@echo "  $(MACOS_DIR)/$(BIN_NAME)#$(BIN_NAME)-macos-aarch64 \\"
	@echo "  $(WINDOWS_DIR)/$(BIN_NAME).exe#$(BIN_NAME)-windows-x86_64.exe"

# 测试
test:
	cargo test --all-features

# 格式化
fmt:
	cargo fmt --all

# Lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# 帮助
help:
	@echo "Available targets:"
	@echo "  build               - Build for current platform"
	@echo "  build-linux         - Cross-compile for Linux x86_64"
	@echo "  build-macos         - Cross-compile for macOS ARM (M1/M2/M3)"
	@echo "  build-macos-intel   - Cross-compile for macOS Intel"
	echo "  build-windows       - Cross-compile for Windows"
	@echo "  build-all           - Build for all platforms"
	@echo "  install             - Install to /usr/local/bin"
	@echo "  run                 - Build and run"
	@echo "  clean               - Clean build artifacts"
	@echo "  release             - Build all and show release commands"
	@echo "  test                - Run tests"
	@echo "  fmt                 - Format code"
	@echo "  lint                - Run clippy"