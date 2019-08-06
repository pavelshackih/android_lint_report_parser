use serde_derive::Deserialize;
use serde_xml_rs::from_reader;
use std::io;
use std::result;

const UNUSED_RESOURCES: &str = "UnusedResources";
const FILE_VERSION: &str = "5";

#[derive(Debug, Deserialize)]
struct Root {
    pub format: String,
    pub by: String,
    #[serde(rename = "issue", default)]
    pub vec: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: String,
    pub message: String,
    pub explanation: String,
    #[serde(rename = "location", default)]
    pub locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub file: String,
}

pub type Result<T> = result::Result<T, CliError>;

pub fn parse(path: &str) -> Result<Vec<Issue>> {
    let file = std::fs::read(path)?;
    let root = from_reader::<_, Root>(file.as_slice())?;
    if FILE_VERSION != root.format {
        println!("Warning: invalid lint file format, supported {}, but actual {}.", FILE_VERSION, root.format);
    }
    let issues: Vec<Issue> = root.vec
        .into_iter()
        .filter(|item| item.id == *UNUSED_RESOURCES)
        .collect();

    Ok(issues)
}

pub enum CliError {
    IoError(io::Error),
    ParseDeError(serde_xml_rs::Error)
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<serde_xml_rs::Error> for CliError {
    fn from(error: serde_xml_rs::Error) -> Self {
        CliError::ParseDeError(error)
    }
}