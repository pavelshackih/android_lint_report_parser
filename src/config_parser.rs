use serde_derive::Deserialize;
use serde_xml_rs::from_reader;
use std::io::{ErrorKind, Error};

const UNUSED_RESOURCES: &str = "UnusedResources";
const FILE_VERSION: &str = "5";

pub fn parse(path: &str) -> Result<Vec<Issue>, Error> {
    let file = std::fs::read(path)?;
    match from_reader::<_, Root>(file.as_slice()) {
        Ok(root) => {
            if FILE_VERSION != root.format {
                println!("Warning: invalid lint file format, supported {}, but actual {}.", FILE_VERSION, root.format);
            }
            let issues: Vec<Issue> = root.vec
                .into_iter()
                .filter(|item| item.id == *UNUSED_RESOURCES)
                .collect();

            Ok(issues)
        },
        Err(e) => {
            println!("Parsing error {:?}", e);
            Err(Error::from(ErrorKind::Other))
        }
    }
}

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