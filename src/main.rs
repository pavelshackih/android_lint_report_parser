use std::path::Path;

mod config_parser;

use config_parser::Issue;
use config_parser::CliError;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Please provide path to lint report file as first argument.");
        return;
    }

    let file = &args[0];

    if let Err(e) = std::fs::File::open(file) {
        println!("Can't open file {:?}: {:?}", file, e.kind());
        return;
    }

    match config_parser::parse(file) {
        Ok(issues) => manage_issues(issues),
        Err(e) => print_error(e),
    }
}

fn print_error(error: config_parser::CliError) {
    match error {
        CliError::IoError(io_error) => println!("Error with lint file: {:?}", io_error),
        CliError::ParseDeError(parsing_error) => println!("Error while parsing file: {:?}", parsing_error),
    }
}

fn manage_issues(vec: Vec<Issue>) {
    for issue in vec {
        for location in issue.locations {
            let path = Path::new(&location.file);
            let resolver = resolve_file(path);
            match resolver {
                Some(resolver) => apply_resolver_for_resource(path, resolver),
                None => println!("Can't find resolver for file: {:?}", path),
            }
        }
    }
}

fn apply_resolver_for_resource(path: &Path, resolver: IssueResolver) {
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
        .map(|v| detect_resolver_by_parent_folder(v))
}

fn detect_resolver_by_parent_folder(name: &str) -> IssueResolver {
    if name.contains("drawable") || name.contains("layout") {
        IssueResolver::RemoveFile
    } else { IssueResolver::Unknown }
}

enum IssueResolver {
    RemoveFile,
    Unknown,
}