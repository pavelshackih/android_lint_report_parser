use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use config_parser::Issue;

use crate::config_parser::Location;

mod config_parser;
mod errors;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Please provide path to lint report file as first argument.");
        return;
    }

    let file = &args[0];

    match config_parser::parse(file) {
        Ok(issues) => apply_issues(issues),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn apply_issues(vec: Vec<Issue>) {
    for issue in vec.into_iter() {
        for location in &issue.locations {
            proceed_issue_for_location(&issue, &location);
        }
    }
}

fn proceed_issue_for_location(issue: &Issue, location: &Location) {
    let path = Path::new(&location.file);
    match resolve_file_by_path(path) {
        Some(resolver) => fix_resource(&issue, location, resolver),
        None => println!("Can't find resolver for file: {:?}", path),
    }
}

fn fix_resource(issue: &Issue, location: &Location, resolver: IssueResolver) {
    let path = Path::new(&location.file);
    match resolver {
        IssueResolver::RemoveFile => match std::fs::remove_file(path) {
            Ok(_) => println!("File {:?} removed.", path),
            Err(e) => println!("Warning: {:?}: {}.", path, e),
        },
        IssueResolver::RemoveTag => remove_attribute(issue, location),
        IssueResolver::Unknown => println!("Unknown resolver for path: {:?}.", path),
    }
}

fn remove_attribute(issue: &Issue, location: &Location) {
    let file = File::open(&location.file).unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let mut counter = 0;
    let mut tmp_vec = Vec::new();
    while reader.read_line(&mut buf).unwrap() != 0 {
        counter += 1;
        if counter != location.line {
            tmp_vec.push(buf.clone());
            buf.clear();
        }
    }

    let mut file = File::create(&location.file).unwrap();
    for line in tmp_vec {
        file.write(line.as_bytes()).unwrap();
    }
    file.flush().unwrap();
}

fn resolve_file_by_path(path: &Path) -> Option<IssueResolver> {
    path.parent()
        .and_then(|v| v.file_name())
        .and_then(|v| v.to_str())
        .map(|v| get_resolver_by_name(v))
}

fn get_resolver_by_name(name: &str) -> IssueResolver {
    match name {
        n if n.contains("drawable") || n.contains("layout") => IssueResolver::RemoveFile,
        n if n.contains("strings") => IssueResolver::RemoveTag,
        _ => IssueResolver::Unknown,
    }
}

#[derive(Debug, PartialEq)]
enum IssueResolver {
    RemoveFile,
    RemoveTag,
    Unknown,
}