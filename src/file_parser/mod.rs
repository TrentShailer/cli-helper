mod module;
mod write;

use std::{
    fs::{self},
    io::{self},
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::FileType;

pub use module::{Module, ModuleError, to_valid_ident};
pub use write::WriteError;

/// Parses files into modules containing an inner parsed type.
pub struct FileParser<T: TryFrom<String>> {
    /// The modules parsed.
    pub modules: Vec<Module<T>>,
}

impl<T: TryFrom<String>> FileParser<T> {
    /// Parse modules from some source.
    /// If source is a directory, this will be top level files in the directory, else it will parse
    /// the single file.
    pub fn parse(source: &Path) -> Result<Self, FileParseError<T::Error>> {
        let metadata = source
            .metadata()
            .map_err(|e| FileParseError::Io(e, source.to_path_buf()))?;

        let file_type = FileType::from(&metadata);

        let mut modules = vec![];
        match file_type {
            FileType::File => {
                let module = Module::try_from(source)
                    .map_err(|e| FileParseError::Parse(e, source.to_path_buf()))?;

                modules.push(module);
            }

            FileType::Directory => {
                let directory = fs::read_dir(source)
                    .map_err(|e| FileParseError::Io(e, source.to_path_buf()))?;

                for entry in directory {
                    let entry =
                        entry.map_err(|e| FileParseError::Io(e, PathBuf::from("Unknown")))?;

                    if !entry.file_type().expect("File must have type").is_file() {
                        continue;
                    }

                    let module = Module::try_from(entry.path().as_path())
                        .map_err(|e| FileParseError::Parse(e, entry.path()))?;

                    modules.push(module)
                }
            }

            file_type => {
                return Err(FileParseError::UnsupportedType(
                    source.to_path_buf(),
                    file_type,
                ));
            }
        };

        modules.sort_by(|a, b| {
            a.source
                .file_name()
                .unwrap()
                .cmp(b.source.file_name().unwrap())
        });

        Ok(Self { modules })
    }
}

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum FileParseError<T> {
    #[error("IO error for '{1:?}': {0}")]
    Io(#[source] io::Error, PathBuf),

    #[error("Unsupported file type for '{0:?}': {1:?}")]
    UnsupportedType(PathBuf, FileType),

    #[error("Failed to parse module '{1:?}': {0}")]
    Parse(#[source] ModuleError<T>, PathBuf),
}
