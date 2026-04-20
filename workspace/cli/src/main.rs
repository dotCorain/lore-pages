//! Lore 文档转换器命令行工具

use lang_framework::{Config, Converter};
use lang_impl::HtmlRenderer;
use lang_parser::MarkdownParser;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置（framework 负责）
    let config = Config::from_file_or_default("Lore.toml");

    let src_dir = Path::new(&config.from_lore_path);
    let dst_dir = Path::new(&config.to_html_path);

    // 创建转换器（cli 只负责组装）
    let converter = Converter::from_config(&config, MarkdownParser, HtmlRenderer);

    // 遍历并转换
    convert_directory(&converter, src_dir, dst_dir)?;

    println!("✅ 转换完成！");
    Ok(())
}

/// 遍历目录并转换所有 .lore 文件
/// 遍历目录并转换所有 `.lore` 文件为 HTML。
///
/// - `converter`: 负责解析与渲染的转换器实例（将解析器和渲染器组合）。
/// - `src_dir`: 源目录，包含 `.lore` 文件和子目录。
/// - `dst_dir`: 目标输出目录，用于保存生成的 `.html` 文件。
///
/// 函数会递归遍历子目录；遇到 I/O 错误会向上返回 `Err`。
fn convert_directory(
    converter: &Converter<MarkdownParser, HtmlRenderer>,
    src_dir: &Path,
    dst_dir: &Path,
) -> std::io::Result<()> {
    if !src_dir.exists() {
        println!("⚠️  源目录不存在: {:?}", src_dir);
        return Ok(());
    }

    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();

        if src_path.is_dir() {
            // 递归处理子目录
            let dir_name = src_path.file_name().unwrap();
            let dst_path = dst_dir.join(dir_name);
            convert_directory(converter, &src_path, &dst_path)?;
        } else if is_lore_file(&src_path) {
            // 处理 .lore 文件
            let content = fs::read_to_string(&src_path)?;
            let title = extract_title(&src_path);
            let html = converter.convert(&content, &title);

            let dst_path = dst_dir
                .join(src_path.file_stem().unwrap())
                .with_extension("html");
            fs::write(&dst_path, html)?;
            println!("✅ 已转换: {:?} -> {:?}", src_path, dst_path);
        }
    }

    Ok(())
}

/// 检查是否为 .lore 文件
/// 判断给定路径是否为 `.lore` 文件（通过扩展名判断）。
fn is_lore_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "lore")
        .unwrap_or(false)
}

/// 从文件路径提取标题（文件名）
/// 从文件路径提取标题（使用文件名，不包含扩展名）。
///
/// 例如 `docs/guide.lore` 会返回 `guide`。当提取失败时返回空字符串。
fn extract_title(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
