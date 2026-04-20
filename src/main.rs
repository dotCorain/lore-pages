use lore_pages::framework::converter::Converter;
use lore_pages::framework::renderer_config::Config;
use lore_pages::parser::MarkdownParser;
use lore_pages::render::HtmlRenderer;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file_or_default("Lore.toml");

    let src_dir = Path::new(&config.from_lore_path);
    let dst_dir = Path::new(&config.to_html_path);

    let converter = Converter::from_config(&config, MarkdownParser, HtmlRenderer);

    convert_directory(&converter, src_dir, dst_dir)?;

    println!("done.");
    Ok(())
}

fn convert_directory(
    converter: &Converter<MarkdownParser, HtmlRenderer>,
    src_dir: &Path,
    dst_dir: &Path,
) -> std::io::Result<()> {
    if !src_dir.exists() {
        println!("源目录不存在: {:?}", src_dir);
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
            let title = extract_title(&src_path);
            let html = converter.convert(&content, &title);

            let dst_path = dst_dir
                .join(src_path.file_stem().unwrap())
                .with_extension("html");
            fs::write(&dst_path, html)?;
            println!("已转换: {:?} -> {:?}", src_path, dst_path);
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

fn extract_title(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
