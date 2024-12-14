use crate::scanner::Node;
use regex::Regex;
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

pub fn compare_nodes(node1: &Node, node2: &Node, report: &mut DiffReport, exclude_pattern: &str) {
    // 如果 `exclude_pattern` 是空字符串，则不排除任何文件或路径
    let regex = if exclude_pattern.is_empty() {
        None
    } else {
        Some(Regex::new(exclude_pattern).unwrap_or_else(|_| Regex::new("^$").unwrap()))
    };

    match (node1, node2) {
        // 比较两个文件
        (
            Node::File {
                name: name1,
                md5: md51,
            },
            Node::File {
                name: name2,
                md5: md52,
            },
        ) => {
            if name1 != name2 {
                report.missing_in_left.push(name1.clone());
                report.missing_in_right.push(name2.clone());
            } else if md51 != md52 {
                // 如果正则表达式为 None，则不排除任何文件
                if let Some(regex) = &regex {
                    if !regex.is_match(name1) {
                        report.different_files.push(name1.clone());
                    }
                } else {
                    report.different_files.push(name1.clone());
                }
            }
        }

        // 比较两个目录
        (
            Node::Directory {
                name: name1,
                children: children1,
            },
            Node::Directory {
                name: name2,
                children: children2,
            },
        ) => {
            if name1 != name2 {
                report.missing_in_left.push(name1.clone());
                report.missing_in_right.push(name2.clone());
            } else {
                // 构建子节点的哈希表（过滤掉匹配的文件或目录）
                let children_map1: HashMap<String, &Node> = children1
                    .iter()
                    .filter(|n| {
                        // 如果正则表达式为 None，则不排除任何文件
                        if let Some(regex) = &regex {
                            !regex.is_match(n.0)
                        } else {
                            true
                        }
                    })
                    .map(|n| (n.0.to_string(), n.1)) // Fix: Use n.1 to get the &Node
                    .collect();
                let children_map2: HashMap<String, &Node> = children2
                    .iter()
                    .filter(|n| {
                        // 如果正则表达式为 None，则不排除任何文件
                        if let Some(regex) = &regex {
                            !regex.is_match(n.0)
                        } else {
                            true
                        }
                    })
                    .map(|n| (n.0.to_string(), n.1))
                    .collect();

                // 遍历第一个目录的子节点
                for (name1, child1) in &children_map1 {
                    match children_map2.get(name1) {
                        Some(child2) => {
                            // 如果两个目录中都有该子节点，递归比较它们
                            compare_nodes(child1, child2, report, exclude_pattern);
                        }
                        None => {
                            // 如果子节点在第二个目录中不存在，记录为右侧缺失
                            report.missing_in_right.push(name1.clone());
                        }
                    }
                }

                // 遍历第二个目录的子节点，检查左侧是否缺失
                for (name2, child2) in &children_map2 {
                    if !children_map1.contains_key(name2) {
                        report.missing_in_left.push(name2.clone());
                    }
                }
            }
        }

        // 文件和目录的类型不匹配
        (Node::File { name: name1, .. }, Node::Directory { name: name2, .. }) => {
            report.missing_in_left.push(name1.clone());
            report.missing_in_right.push(name2.clone());
        }
        (Node::Directory { name: name1, .. }, Node::File { name: name2, .. }) => {
            report.missing_in_left.push(name1.clone());
            report.missing_in_right.push(name2.clone());
        }
    }
}
