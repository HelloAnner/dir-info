/*
@author Anner
@since 1.0
Created on 2024/12/14
*/

use bincode;
use crate::scanner::Node;

pub fn save_to_file(node: &Node, file_path: &str) {
    let encoded: Vec<u8> = bincode::serialize(node).unwrap();
    std::fs::write(file_path, encoded).unwrap();
}