use std::path::Path;
use std::env;

use config_parser::Issue;

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
    for issue in vec {
        for location in issue.locations {
            let path = Path::new(&location.file);
            match resolve_file(path) {
                Some(resolver) => fix_res(path, resolver),
                None => println!("Can't find resolver for file: {:?}", path),
            }
        }
    }
}

fn fix_res(path: &Path, resolver: IssueResolver) {
    match resolver {
        IssueResolver::RemoveFile => match std::fs::remove_file(path) {
            Ok(_) => println!("File {:?} removed.", path),
            Err(e) => println!("Warning: {:?}: {}.", path, e),
        },
        IssueResolver::Unknown => println!("Unknown resolver for path: {:?}.", path),
    }
}

fn resolve_file(path: &Path) -> Option<IssueResolver> {
    path.parent()
        .and_then(|v| v.file_name())
        .and_then(|v| v.to_str())
        .map(|v| get_resolver_by_name(v))
}

fn get_resolver_by_name(name: &str) -> IssueResolver {
    match name {
        n if n.contains("drawable") || n.contains("layout") => IssueResolver::RemoveFile,
        _ => IssueResolver::Unknown,
    }
}

#[derive(Debug, PartialEq)]
enum IssueResolver {
    RemoveFile,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_resolver() {
        assert_eq!(get_resolver_by_name("blabla"), IssueResolver::Unknown);
        assert_eq!(get_resolver_by_name("/drawable/"), IssueResolver::RemoveFile);
        assert_eq!(get_resolver_by_name("layout//"), IssueResolver::RemoveFile);
    }
}