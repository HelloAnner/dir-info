/*
扫描目录结构

@author Anner
@since 1.0
Created on 2024/12/14
*/

use md5;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    File { name: String, md5: String },
    Directory { name: String, children: Vec<Node> },
}

impl Node {
    pub fn get_name(&self) -> &str {
        match self {
            Node::File { name, .. } => name,
            Node::Directory { name, .. } => name,
        }
    }
}

pub fn scan_directory(path: &Path) -> Node {
    if path.is_file() {
        let mut file = File::open(path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let md5 = format!("{:x}", md5::compute(&contents));
        Node::File { name: path.file_name().unwrap().to_str().unwrap().to_string(), md5 }
    } else if path.is_dir() {
        let children: Vec<Node> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path() != path)
            .map(|entry| {
                let rel_path = entry.path().strip_prefix(path.parent().unwrap()).unwrap();
                let node_path = Path::new(rel_path);
                scan_directory(node_path)
            })
            .collect();
        Node::Directory { name: path.file_name().unwrap().to_str().unwrap().to_string(), children }
    } else {
        panic!("Unknown file type");
    }
}