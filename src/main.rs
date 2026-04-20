use lore_pages::framework::category_config::CategoryConfig;
use lore_pages::framework::converter::CategoryConverter;
use lore_pages::framework::parser_config::ParserConfig;
use lore_pages::framework::renderer_config::RenderConfig;
use lore_pages::parser::MarkdownParser;
use lore_pages::render::HtmlRenderer;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer_config = match RenderConfig::from_file("Lore.toml") {
        Ok(cfg) => {
            println!(
                "✓ 已加载配置: from_lore_path={}, to_html_path={}",
                cfg.from_lore_path, cfg.to_html_path
            );
            cfg
        }
        Err(e) => {
            eprintln!("⚠ 配置加载失败: {}", e);
            eprintln!("  使用默认配置");
            RenderConfig::default()
        }
    };

    let src_dir = Path::new(&renderer_config.from_lore_path);
    let dst_dir = Path::new(&renderer_config.to_html_path);

    // Create configs (parser config may be loaded from Lore.toml)
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

    convert_directory(&converter, src_dir, dst_dir)?;

    println!("done.");
    Ok(())
}

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
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "lore")
        .unwrap_or(false)
}
