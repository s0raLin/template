use anyhow::Result;
use serde_json::Value;

pub fn get(url: &str) -> Result<()> {
    println!("发送 GET 请求到: {}", url);
    println!("提示: 需要添加 reqwest 依赖才能实际发送 HTTP 请求");
    println!("运行: cargo add reqwest --features json");
    Ok(())
}

pub fn parse_json(json_str: &str) -> Result<()> {
    let value: Value = serde_json::from_str(json_str)?;
    let pretty = serde_json::to_string_pretty(&value)?;
    println!("{}", pretty);
    Ok(())
}
