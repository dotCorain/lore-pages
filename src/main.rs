use lore_pages::framework::category_config::CategoryConfig;
use lore_pages::framework::converter::CategoryConverter;
use lore_pages::framework::parser_config::ParserConfig;
use lore_pages::framework::renderer_config::RenderConfig;
use lore_pages::parser::LorePagesParser;
use lore_pages::render::HtmlRenderer;
// 标准库：文件读写与路径处理

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
// 程序入口
// 返回 `Result<(), Box<dyn std::error::Error>>`：
// - `Box<dyn std::error::Error>` 是一个“装箱”的 trait 对象，用来统一不同错误类型，便于主函数返回各种可能的错误

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 尝试从 `Lore.toml` 加载渲染配置，失败则回退到默认配置
    let renderer_config = match RenderConfig::from_file("Lore.toml") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load Lore.toml: {}", e);
            eprintln!("using default configuration.");
            RenderConfig::default()
        }
    };

    let src_dir = Path::new(&renderer_config.from_lore_path);
    let dst_dir = Path::new(&renderer_config.to_html_path);

    // 创建配置
    let category_config = CategoryConfig::default();
    let parser_config = ParserConfig::from_file_or_default("Lore.toml");

    let parser = LorePagesParser;
    let renderer = HtmlRenderer;

    let converter = CategoryConverter::from_config(
        parser,
        renderer,
        &category_config,
        &renderer_config,
        &parser_config,
    );

    // 优先使用 git ls-files 快速获取文件列表，失败则回退到目录遍历
    match collect_lore_files_via_git(src_dir) {
        Some(files) => {
            println!("Found {} .lore files via git ls-files", files.len());
            convert_files(&converter, src_dir, dst_dir, &files)?;
        }
        None => {
            convert_directory(&converter, src_dir, dst_dir)?;
        }
    }

    println!("done.");
    Ok(())
}

// 递归转换目录：
// - `'a` 是转换器生命周期，`P`/`R` 是泛型，绑定到 `Parser`/`Renderer` trait
// - 使用 `?` 运算符将 I/O 错误向上传播，函数返回 `std::io::Result<()>`
fn convert_directory<'a, P, R>(
    converter: &CategoryConverter<'a, P, R>,
    src_dir: &Path,
    dst_dir: &Path,
) -> std::io::Result<()>
where
    P: lore_pages::framework::parser::Parser,
    R: lore_pages::framework::renderer::Renderer,
{
    if !src_dir.exists() {
        println!("Not found {:?}", src_dir);
        return Ok(());
    }

    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();

        if src_path.is_dir() {
            let dir_name = src_path.file_name().unwrap();
            let dst_path = dst_dir.join(dir_name);
            convert_directory(converter, &src_path, &dst_path)?;
        } else if is_lore_file(&src_path) {
            let content = fs::read_to_string(&src_path)?;
            let html = converter.convert_simple(&content);

            let dst_path = dst_dir
                .join(src_path.file_stem().unwrap())
                .with_extension("html");
            fs::write(&dst_path, html)?;
            println!("{:?} -> {:?}", src_path, dst_path);
        }
    }

    Ok(())
}

fn is_lore_file(path: &Path) -> bool {
    // 判断扩展名是否为 "lore"
    // `extension()` 返回 `Option<OsStr>`，后面链式转换为 `Option<&str>` 再判断是否等于 "lore"
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "lore")
        .unwrap_or(false)
}

/// 使用 `git ls-files` 快速收集 `src_dir` 下所有 `.lore` 文件。
/// 相比文件系统遍历，git 索引在大目录下速度更快。
/// 返回 `None` 表示 git 不可用或目录不在 git 仓库中，此时应回退到目录遍历。
fn collect_lore_files_via_git(src_dir: &Path) -> Option<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(["-C", src_dir.to_str()?, "ls-files", "--", "*.lore"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let files: Vec<PathBuf> = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| src_dir.join(line.trim()))
        .collect();

    if files.is_empty() {
        None
    } else {
        Some(files)
    }
}

/// 将给定的 `.lore` 文件列表转换到目标目录，保留子目录结构。
fn convert_files<'a, P, R>(
    converter: &CategoryConverter<'a, P, R>,
    src_dir: &Path,
    dst_dir: &Path,
    files: &[PathBuf],
) -> std::io::Result<()>
where
    P: lore_pages::framework::parser::Parser,
    R: lore_pages::framework::renderer::Renderer,
{
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }

    for src_path in files {
        if !src_path.exists() {
            eprintln!("Warning: file not found, skipping: {:?}", src_path);
            continue;
        }

        // 计算相对于 src_dir 的路径
        let rel_path = match src_path.strip_prefix(src_dir) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Warning: cannot compute relative path for {:?}", src_path);
                continue;
            }
        };

        // 构建目标路径，保持子目录结构
        let dst_path = if let Some(parent) = rel_path.parent() {
            let dir = dst_dir.join(parent);
            fs::create_dir_all(&dir)?;
            dir.join(rel_path.file_stem().unwrap()).with_extension("html")
        } else {
            dst_dir
                .join(rel_path.file_stem().unwrap())
                .with_extension("html")
        };

        let content = fs::read_to_string(src_path)?;
        let html = converter.convert_simple(&content);
        fs::write(&dst_path, html)?;
        println!("{:?} -> {:?}", src_path, dst_path);
    }

    Ok(())
}
