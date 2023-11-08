use regex::{Captures, Regex};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn strings_to_json(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let json = convert_to_json(&input)?;
    let mut output = File::create(output)?;
    write!(output, "{}", json)?;
    Ok(())
}

fn convert_to_json(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
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
            // 将有序的%n$(@|d|i|u|f|c|s)转换为{tn}
            let re = Regex::new(r"%(\d+)\$(@|d|i|u|f|c|s)").ok()?;
            value = re
                .replace_all(&value, |caps: &Captures| format!("{{t{}}}", &caps[1]))
                .to_string();
            // 将无序的%(@|d|i|u|f|c|s)转换为{t1}，%(@|d|i|u|f|c|s)转换为{t2}等
            let re_unordered = Regex::new(r"%(@|d|i|u|f|c|s)").ok()?;
            let mut index = 1;
            value = re_unordered
                .replace_all(&value, |_caps: &Captures| {
                    let replacement = format!("{{t{}}}", index);
                    index += 1;
                    replacement
                })
                .to_string();

            return Some((key, value));
        }
    }
    None
}
