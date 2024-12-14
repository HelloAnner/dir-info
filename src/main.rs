mod scanner;
mod serializer;
mod deserializer;
mod comparator;

use clap::{App, Arg, SubCommand};
use comparator::{compare_nodes, DiffReport};
use deserializer::load_from_file;
use scanner::scan_directory;
use serializer::save_to_file;
use std::path::Path;

fn main() {
    let matches = App::new("DirInfo")
        .subcommand(
            SubCommand::with_name("scan")
                .arg(Arg::with_name("directory").required(true).takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("compare")
                .arg(Arg::with_name("file1").required(true).takes_value(true))
                .arg(Arg::with_name("file2").required(true).takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("show")
                .arg(Arg::with_name("file").required(true).takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("compare_dir")
                .arg(Arg::with_name("file").required(true).takes_value(true))
                .arg(Arg::with_name("directory").required(true).takes_value(true))
        ).get_matches();

    match matches.subcommand() {
        ("scan", Some(scan_matches)) => {
            let path = scan_matches.value_of("directory").unwrap();
            let node = scan_directory(Path::new(path));
            save_to_file(&node, "output.bin");
            println!("Directory information saved to 'output.bin'");
        }
        ("compare", Some(compare_matches)) => {
            let file1 = compare_matches.value_of("file1").unwrap();
            let file2 = compare_matches.value_of("file2").unwrap();
            let node1 = load_from_file(file1);
            let node2 = load_from_file(file2);
            let mut report = DiffReport::new();
            compare_nodes(&node1, &node2, &mut report);
            print_diff_report(&report);
        }
        ("show", Some(extract_matches)) => {
            let file = extract_matches.value_of("file").unwrap();
            let node = load_from_file(file);
            println!("Extracted directory structure:");
            print_node(&node, 0);
        }
        ("compare_dir", Some(compare_dir_matches)) => {
            let file = compare_dir_matches.value_of("file").unwrap();
            let directory = compare_dir_matches.value_of("directory").unwrap();
            let node_file = load_from_file(file);
            let node_dir = scan_directory(Path::new(directory));
            let mut report = DiffReport::new();
            compare_nodes(&node_file, &node_dir, &mut report);
            print_diff_report(&report);
        }
        _ => {
            println!("No subcommand provided. Use --help for usage information.");
        }
    }
}

fn print_diff_report(report: &DiffReport) {
    println!("Differences found:");
    if !report.missing_in_left.is_empty() {
        println!("\nFiles/directories missing in left:");
        for path in &report.missing_in_left {
            println!("  - {}", path);
        }
    }
    if !report.missing_in_right.is_empty() {
        println!("\nFiles/directories missing in right:");
        for path in &report.missing_in_right {
            println!("  - {}", path);
        }
    }
    if !report.different_files.is_empty() {
        println!("\nFiles with different content:");
        for path in &report.different_files {
            println!("  - {}", path);
        }
    }
    if report.missing_in_left.is_empty()
        && report.missing_in_right.is_empty()
        && report.different_files.is_empty()
    {
        println!("No differences found.");
    }
}

fn print_node(node: &scanner::Node, indent: usize) {
    match node {
        scanner::Node::File { name, md5 } => {
            println!("{:indent$}File: {} (MD5: {})", "", name, md5, indent = indent);
        }
        scanner::Node::Directory { name, children } => {
            println!("{:indent$}Directory: {}", "", name, indent = indent);
            for child in children {
                print_node(child, indent + 2);
            }
        }
    }
}