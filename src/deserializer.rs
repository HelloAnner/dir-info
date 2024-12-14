/*
@author Anner
@since 1.0
Created on 2024/12/14
*/

use bincode;
use crate::scanner::Node;

pub fn load_from_file(file_path: &str) -> Node {
    let data = std::fs::read(file_path).unwrap();
    bincode::deserialize(&data[..]).unwrap()
}