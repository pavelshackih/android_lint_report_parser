use std::io;
use std::result;

pub type Result<T> = result::Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    IoError(io::Error),
    ParseDeError(serde_xml_rs::Error),
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
