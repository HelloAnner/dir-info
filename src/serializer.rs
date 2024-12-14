/*
@author Anner
@since 1.0
Created on 2024/12/14
*/

use crate::scanner::Node;
use std::borrow::Cow;
use std::fs;
use std::path::Path;

pub(crate) fn save_to_file(node: &Node, scan_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 获取目录路径的名称（basename）
    let dir_name: Cow<str> = Path::new(scan_path)
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("output"))
        .to_string_lossy();

    // 生成输出文件名，例如 "扫描的目录名称.info"
    let output_file_name = &format!("{}", dir_name);

    // 将节点信息保存到文件
    let serialized: Vec<u8> = bincode::serialize(node)?;
    fs::write(output_file_name, serialized)?;

    println!("Directory information saved to {}", output_file_name);
    Ok(())
}
