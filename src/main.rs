use clap::Parser;
use regex::{Captures, Regex};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // 获取当前目录
    let path = args.input;
    // 执行转换操作
    let json = strings_to_json(&path)?;
    // 输出到文件
    let path = &args.output;
    let mut output = File::create(path)?;
    write!(output, "{}", json)?;
    println!("Conversion complete!");
    Ok(())
}

fn strings_to_json(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    println!("Converting {:?}", path);

    // 打开.strings文件
    let file = File::open(path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    // 创建一个空的JSON对象
    let mut json = json!({});

    for line in reader.lines() {
        if let Ok(line_str) = line {
            // 解析每一行，提取出key和value
            if let Some((key, value)) = parse_line(&line_str) {
                // 添加到JSON对象中
                json[key] = Value::String(value);
            }
        }
    }

    let json = serde_json::to_string_pretty(&json)?;
    Ok(json)
}

fn parse_line(line: &str) -> Option<(String, String)> {
    if !line.starts_with("//") && !line.is_empty() {
        let parts: Vec<&str> = line.splitn(2, " = ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim_end_matches(';').trim_matches('"').to_string();
            let mut value = parts[1].trim_end_matches(';').trim_matches('"').to_string();

            // 将%1$@，%2$@等转换为${t1}，${t2}等
            let re = Regex::new(r"%(\d+)\$@").unwrap();
            value = re
                .replace_all(&value, |caps: &Captures| format!("${{t{}}}", &caps[1]))
                .to_string();

            return Some((key, value));
        }
    }
    None
}
