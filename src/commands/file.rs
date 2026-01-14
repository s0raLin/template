use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn read(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)
        .context(format!("无法读取文件: {}", path))?;
    println!("{}", content);
    Ok(())
}

pub fn write(path: &str, content: &str) -> Result<()> {
    fs::write(path, content)
        .context(format!("无法写入文件: {}", path))?;
    println!("✓ 已写入文件: {}", path);
    Ok(())
}

pub fn list(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    if !path.exists() {
        anyhow::bail!("目录不存在: {}", dir);
    }
    
    println!("📁 {}", dir);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = if entry.file_type()?.is_dir() { "📁" } else { "📄" };
        println!("  {} {}", file_type, entry.file_name().to_string_lossy());
    }
    Ok(())
}
