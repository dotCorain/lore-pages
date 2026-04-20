use lore_pages::framework::category_config::CategoryConfig;
use lore_pages::framework::converter::CategoryConverter;
use lore_pages::framework::parser_config::ParserConfig;
use lore_pages::framework::renderer_config::RenderConfig;
use lore_pages::parser::MarkdownParser;
use lore_pages::render::HtmlRenderer;
// 标准库：文件读写与路径处理
use std::fs;
use std::path::Path;

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

    let parser = MarkdownParser;
    let renderer = HtmlRenderer;

    let converter = CategoryConverter::from_config(
        parser,
        renderer,
        &category_config,
        &renderer_config,
        &parser_config,
    );

    // 将源目录递归转换到目标目录
    convert_directory(&converter, src_dir, dst_dir)?;

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
            println!("✓ 已转换: {:?} -> {:?}", src_path, dst_path);
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
