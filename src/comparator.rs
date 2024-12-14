/*
@author Anner
@since 1.0
Created on 2024/12/14
*/
use crate::scanner::Node;
use std::collections::HashMap;

#[derive(Default)]
pub struct DiffReport {
    pub missing_in_left: Vec<String>,
    pub missing_in_right: Vec<String>,
    pub different_files: Vec<String>,
}

impl DiffReport {
    pub fn new() -> Self {
        DiffReport {
            missing_in_left: Vec::new(),
            missing_in_right: Vec::new(),
            different_files: Vec::new(),
        }
    }
}


pub fn compare_nodes(node1: &Node, node2: &Node, report: &mut DiffReport) {
    match (node1, node2) {
        (Node::File { name: name1, md5: md51 }, Node::File { name: name2, md5: md52 }) => {
            if name1 != name2 {
                report.missing_in_left.push(name1.clone());
                report.missing_in_right.push(name2.clone());
            } else if md51 != md52 {
                report.different_files.push(name1.clone());
            }
        }
        (Node::Directory { name: name1, children: children1 }, Node::Directory { name: name2, children: children2 }) => {
            if name1 != name2 {
                report.missing_in_left.push(name1.clone());
                report.missing_in_right.push(name2.clone());
            } else {
                let children_map1: HashMap<String, &Node> = children1.iter().map(|n| (n.get_name().to_string(), n)).collect();
                let children_map2: HashMap<String, &Node> = children2.iter().map(|n| (n.get_name().to_string(), n)).collect();

                for child in children1 {
                    if let Some(child2) = children_map2.get(child.get_name()) {
                        compare_nodes(child, child2, report);
                    } else {
                        report.missing_in_right.push(child.get_name().to_string());
                    }
                }

                for child in children2 {
                    if !children_map1.contains_key(child.get_name()) {
                        report.missing_in_left.push(child.get_name().to_string());
                    }
                }
            }
        }
        (Node::File { .. }, Node::Directory { .. }) => {
            report.missing_in_left.push("File".to_string());
            report.missing_in_right.push("Directory".to_string());
        }
        (Node::Directory { .. }, Node::File { .. }) => {
            report.missing_in_left.push("Directory".to_string());
            report.missing_in_right.push("File".to_string());
        }
    }
}