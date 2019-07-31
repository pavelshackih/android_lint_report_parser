use std::path::Path;

use serde_derive::Deserialize;
use serde_xml_rs::from_reader;

const SUPPORTED_FORMAT: &str = "4";
const UNUSED_RESOURCES: &str = "UnusedResources";

fn main() {
    let file = std::fs::read("./lint.xml").unwrap();
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
        }
    }
}

fn resolve_file(path: &Path) -> Option<IssueResolver> {
    path.parent()
        .and_then(|v| v.file_name())
        .and_then(|v| v.to_str())
        .map(|v| detect_resolver_by_folder(v))
}

fn detect_resolver_by_folder(name: &str) -> IssueResolver {
    if name.contains("drawable") || name.contains("layout") {
        IssueResolver::RemoveFile
    } else if name == "values" {
        IssueResolver::RemoveTag
    } else { IssueResolver::Unknown }
}

enum IssueResolver {
    RemoveFile,
    RemoveTag,
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