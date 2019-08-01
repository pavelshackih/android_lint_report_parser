use std::path::Path;

use serde_derive::Deserialize;
use serde_xml_rs::from_reader;

const SUPPORTED_FORMAT: &str = "5";
const UNUSED_RESOURCES: &str = "UnusedResources";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    assert!(args.len() > 1);
    let file = &args[1];

    if let Err(e) = std::fs::File::open(file) {
        println!("Can't open file {:?}: {:?}", file, e.kind());
        return;
    }

    let file = std::fs::read(file).unwrap();
    let root: Root = from_reader(file.as_slice()).unwrap();

    if root.format != *SUPPORTED_FORMAT {
        panic!("Invalid lint file format, supported {}, but actual {}", SUPPORTED_FORMAT, root.format);
    }

    let issues: Vec<Issue> = root.vec
        .into_iter()
        .filter(|item| item.id == *UNUSED_RESOURCES)
        .collect();

    manage_issues(issues);
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
            Ok(_) => println!("File {:?} removed", path),
            Err(_) => println!("Can't remove file {:?}", path),
        },
        IssueResolver::Unknown => println!("Unknown resolver for path: {:?}", path),
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

#[derive(Debug, Deserialize)]
struct Root {
    format: String,
    by: String,
    #[serde(rename = "issue", default)]
    vec: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
struct Issue {
    id: String,
    message: String,
    explanation: String,
    #[serde(rename = "location", default)]
    locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
struct Location {
    file: String,
}