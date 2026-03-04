use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Could not read file on disk: '{0}'")]
    NotExist(String),
}

#[derive(Debug)]
pub struct EXRMetadata {
    pub file_format_version: usize,
}

pub fn parse_metadata(p: impl AsRef<Path>) -> Result<EXRMetadata, ParsingError> {
    let file_path = p.as_ref();
    log::info!("Reading metadata of '{}'", file_path.display());

    if !file_path.exists() {
        return Err(ParsingError::NotExist(file_path.display().to_string()));
    }

    // FIXME: Hardcoded just for initial demo purposes
    let metadata = EXRMetadata {
        file_format_version: 2,
    };

    Ok(metadata)
}

pub fn print_metadata(meta: EXRMetadata) {
    println!("{meta:?}");
}
