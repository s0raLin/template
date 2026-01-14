use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;

#[derive(Parser)]
#[command(name = "mycli")]
#[command(about = "一个功能丰富的 Rust CLI 工具", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 问候用户
    Greet {
        /// 要问候的名字
        #[arg(short, long)]
        name: String,
        /// 重复次数
        #[arg(short, long, default_value = "1")]
        times: u32,
    },
    
    /// 数学计算
    Math {
        #[command(subcommand)]
        operation: MathOps,
    },
    
    /// 文件操作
    File {
        #[command(subcommand)]
        operation: FileOps,
    },
    
    /// HTTP 工具
    Http {
        #[command(subcommand)]
        operation: HttpOps,
    },
}

#[derive(Subcommand)]
enum MathOps {
    /// 加法
    Add { a: i32, b: i32 },
    /// 减法
    Sub { a: i32, b: i32 },
    /// 乘法
    Mul { a: i32, b: i32 },
    /// 除法
    Div { a: f64, b: f64 },
}

#[derive(Subcommand)]
enum FileOps {
    /// 读取文件内容
    Read {
        /// 文件路径
        path: String,
    },
    /// 写入文件
    Write {
        /// 文件路径
        path: String,
        /// 要写入的内容
        content: String,
    },
    /// 列出目录内容
    List {
        /// 目录路径
        #[arg(default_value = ".")]
        dir: String,
    },
}

#[derive(Subcommand)]
enum HttpOps {
    /// 发送 GET 请求
    Get {
        /// URL 地址
        url: String,
    },
    /// 格式化 JSON
    Json {
        /// JSON 字符串
        data: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name, times } => {
            commands::greet::execute(&name, times)?;
        }
        
        Commands::Math { operation } => {
            match operation {
                MathOps::Add { a, b } => {
                    let result = commands::math::add(a, b)?;
                    println!("{} + {} = {}", a, b, result);
                }
                MathOps::Sub { a, b } => {
                    let result = commands::math::subtract(a, b)?;
                    println!("{} - {} = {}", a, b, result);
                }
                MathOps::Mul { a, b } => {
                    let result = commands::math::multiply(a, b)?;
                    println!("{} × {} = {}", a, b, result);
                }
                MathOps::Div { a, b } => {
                    let result = commands::math::divide(a, b)?;
                    println!("{} ÷ {} = {}", a, b, result);
                }
            }
        }
        
        Commands::File { operation } => {
            match operation {
                FileOps::Read { path } => {
                    commands::file::read(&path)?;
                }
                FileOps::Write { path, content } => {
                    commands::file::write(&path, &content)?;
                }
                FileOps::List { dir } => {
                    commands::file::list(&dir)?;
                }
            }
        }
        
        Commands::Http { operation } => {
            match operation {
                HttpOps::Get { url } => {
                    commands::http::get(&url)?;
                }
                HttpOps::Json { data } => {
                    commands::http::parse_json(&data)?;
                }
            }
        }
    }

    Ok(())
}
