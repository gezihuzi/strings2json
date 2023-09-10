use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

fn main() {
    // 获取当前目录
    let mut current_dir = std::env::current_dir().expect("Failed to get current directory");
    current_dir.push("strings");
    // 遍历当前目录下的所有文件和目录
    for entry in fs::read_dir(&current_dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            // 检查是否是.strings文件
            if let Some(extension) = path.extension() {
                if extension == "strings" {
                    // 执行转换操作
                    convert_to_json(&path);
                }
            }
        }
    }

    println!("Conversion complete!");
}

fn convert_to_json(file_path: &std::path::Path) {
    println!("Converting {:?}", file_path);

    // 打开.strings文件
    let file = File::open(file_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    // 创建一个空的JSON对象
    let mut json_obj = json!({});

    for line in reader.lines() {
        if let Ok(line_str) = line {
            // 解析每一行，提取出key和value
            if let Some((key, value)) = parse_line(&line_str) {
                // 添加到JSON对象中
                json_obj[key] = Value::String(value);
            }
        }
    }

    // 将JSON对象写入输出文件
    let output_file_name = file_path.with_extension("json");
    let mut output = File::create(output_file_name).expect("Failed to create output file");
    let json_data = serde_json::to_string_pretty(&json_obj).expect("Failed to serialize JSON");
    write!(output, "{}", json_data).expect("Failed to write JSON to file");
}

fn parse_line(line: &str) -> Option<(String, String)> {
    if !line.starts_with("//") && !line.is_empty() {
        let parts: Vec<&str> = line.splitn(2, " = ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim_end_matches(';').trim_matches('"').to_string();
            let value = parts[1].trim_matches('"').to_string();
            return Some((key, value));
        }
    }
    None
}
