use md5;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tempfile::tempdir;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    File {
        name: String,
        md5: String,
    },
    Directory {
        name: String,
        children: HashMap<String, Node>,
    },
}

pub fn scan_directory(
    path: &Path,
    exclude_pattern: &str,
) -> Result<Node, Box<dyn std::error::Error>> {
    let regex = if exclude_pattern.is_empty() {
        None
    } else {
        Some(Regex::new(exclude_pattern).unwrap_or_else(|_| Regex::new("^$").unwrap()))
    };

    if path.is_file() {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let md5 = format!("{:x}", md5::compute(&contents));
        return Ok(Node::File {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            md5,
        });
    }

    if path.is_dir() {
        let mut children: HashMap<String, Node> = HashMap::new();
        for entry in WalkDir::new(path)
            .min_depth(1) // Skip the root directory itself, only recurse into sub-items
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            let node_path = entry.path();
            if let Some(ref regex) = regex {
                if regex.is_match(node_path.to_str().unwrap_or("")) {
                    continue;
                }
            }

            if let Ok(node) = scan_directory(node_path, exclude_pattern) {
                let name = node.get_name().to_string();
                // Insert the node into the map if it doesn't already exist
                children.insert(name, node);
            }
        }

        return Ok(Node::Directory {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            children,
        });
    }

    Err(format!("Unknown file type - {}", path.display()).into())
}

impl Node {
    pub fn get_name(&self) -> &str {
        match self {
            Node::File { name, .. } => name,
            Node::Directory { name, .. } => name,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Node::File {
                    name: name1,
                    md5: md51,
                },
                Node::File {
                    name: name2,
                    md5: md52,
                },
            ) => name1 == name2 && md51 == md52,
            (
                Node::Directory {
                    name: name1,
                    children: children1,
                },
                Node::Directory {
                    name: name2,
                    children: children2,
                },
            ) => name1 == name2 && children1 == children2,
            _ => false,
        }
    }
}

#[test]
fn test_scan_directory() {
    // 创建临时目录
    let dir = tempdir().expect("Failed to create temp dir");
    let dir_path = dir.path();

    // 创建多层目录结构
    fs::create_dir(dir_path.join("dir1")).expect("Failed to create dir1");
    fs::create_dir(dir_path.join("dir1/dir2")).expect("Failed to create dir2");
    fs::write(dir_path.join("file1.txt"), "content1").expect("Failed to create file1.txt");
    fs::write(dir_path.join("dir1/file2.txt"), "content2").expect("Failed to create file2.txt");
    fs::write(dir_path.join("dir1/dir2/file3.txt"), "content3")
        .expect("Failed to create file3.txt");

    // 调用 scan_directory 函数
    let result = scan_directory(dir_path, "").expect("Failed to scan directory");

    // 构建预期的 Node 结构
    let mut expected_children = HashMap::new();
    expected_children.insert(
        "file1.txt".to_string(),
        Node::File {
            name: "file1.txt".to_string(),
            md5: format!("{:x}", md5::compute("content1")),
        },
    );

    let mut dir1_children = HashMap::new();
    dir1_children.insert(
        "file2.txt".to_string(),
        Node::File {
            name: "file2.txt".to_string(),
            md5: format!("{:x}", md5::compute("content2")),
        },
    );

    let mut dir2_children = HashMap::new();
    dir2_children.insert(
        "file3.txt".to_string(),
        Node::File {
            name: "file3.txt".to_string(),
            md5: format!("{:x}", md5::compute("content3")),
        },
    );

    dir1_children.insert(
        "dir2".to_string(),
        Node::Directory {
            name: "dir2".to_string(),
            children: dir2_children,
        },
    );

    expected_children.insert(
        "dir1".to_string(),
        Node::Directory {
            name: "dir1".to_string(),
            children: dir1_children,
        },
    );

    let expected_node = Node::Directory {
        name: dir_path.file_name().unwrap().to_str().unwrap().to_string(),
        children: expected_children,
    };

    // 验证结果
    assert_eq!(result, expected_node);
}
