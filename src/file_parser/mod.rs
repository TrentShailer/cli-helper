mod module;
#[cfg(feature = "generate")]
mod write;

use core::{error::Error, fmt};
use std::{
    fs::{self},
    io::{self},
    path::{Path, PathBuf},
};

use crate::FileType;

pub use module::{Module, to_valid_ident};
#[cfg(feature = "generate")]
pub use write::{WriteError, WriteErrorKind};

/// A type can be parsed from some source using some state.
pub trait ParseFrom<Source, State: ?Sized>: Sized {
    /// Error type to return if the source could not be parsed.
    type Error: Error + 'static;

    /// Try parse the source into an instance of self.
    fn parse(source: Source, state: &mut State) -> Result<Self, Self::Error>;
}

/// Parses files into modules containing an inner parsed type.
pub struct FileParser<State, T: ParseFrom<String, State>> {
    /// The modules parsed.
    pub modules: Vec<Module<State, T>>,
}

impl<State, T: ParseFrom<String, State>> FileParser<State, T> {
    /// Parse modules from some source.
    /// If source is a directory, this will be top level files in the directory, else it will parse
    /// the single file.
    pub fn parse(source: &Path, state: &mut State) -> Result<Self, ParseFileError<T::Error>> {
        let metadata = source.metadata().map_err(|e| ParseFileError {
            kind: ParseFileErrorKind::ReadSourceMetadata {
                source: e,
                path: source.to_path_buf(),
            },
        })?;

        let file_type = FileType::from(&metadata);

        let mut modules = vec![];
        match file_type {
            FileType::File => {
                let module = Module::parse(source, state)?;

                modules.push(module);
            }

            FileType::Directory => {
                let directory = fs::read_dir(source).map_err(|e| ParseFileError {
                    kind: ParseFileErrorKind::ReadDirectory {
                        source: e,
                        path: source.to_path_buf(),
                    },
                })?;

                for entry in directory {
                    let entry = entry.map_err(|e| ParseFileError {
                        kind: ParseFileErrorKind::ReadDirectory {
                            source: e,
                            path: source.join("?").to_path_buf(),
                        },
                    })?;

                    if !entry.file_type().expect("File must have type").is_file() {
                        continue;
                    }

                    let module = Module::parse(entry.path().as_path(), state)?;

                    modules.push(module)
                }
            }

            file_type => {
                return Err(ParseFileError {
                    kind: ParseFileErrorKind::UnsupportedFileType {
                        file_type,
                        path: source.to_path_buf(),
                    },
                });
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

#[derive(Debug)]
#[non_exhaustive]
/// Failed to parse the file or directory.
pub struct ParseFileError<E: Error + 'static> {
    /// The error variants.
    pub kind: ParseFileErrorKind<E>,
}
impl<E: Error> fmt::Display for ParseFileError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error while parsing the source file/directory")
    }
}
impl<E: Error> Error for ParseFileError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// The variants for failing to parse a file or directory.
pub enum ParseFileErrorKind<E: Error + 'static> {
    #[non_exhaustive]
    /// Failed to read the source file metadata.
    ReadSourceMetadata {
        /// The source IO error.
        source: io::Error,
        /// The source file/directory path.
        path: PathBuf,
    },

    #[non_exhaustive]
    /// Failed to read the source directory.
    ReadDirectory {
        /// The source IO error.
        source: io::Error,
        /// The source directory path.
        path: PathBuf,
    },

    #[non_exhaustive]
    /// The source path pointed to an unsupported filetype.
    UnsupportedFileType {
        /// The filetype of the source.
        file_type: FileType,
        /// The path to the source.
        path: PathBuf,
    },

    #[non_exhaustive]
    /// Failed to read one of the source files.
    ReadFile {
        /// The source IO error.
        source: io::Error,
        /// The path to the source.
        path: PathBuf,
    },

    #[non_exhaustive]
    /// Failed to parse one of the files.
    ParseContents {
        /// The source module parse error.
        source: E,
        /// The path to the source.
        path: PathBuf,
    },
}
impl<E: Error> fmt::Display for ParseFileErrorKind<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ReadSourceMetadata { path, .. } => write!(
                f,
                "failed reading the metadata for `{}`",
                path.to_string_lossy()
            ),
            Self::ReadDirectory { path, .. } => {
                write!(
                    f,
                    "failed reading the directory `{}`",
                    path.to_string_lossy()
                )
            }
            Self::UnsupportedFileType { file_type, path } => {
                write!(
                    f,
                    "source file type `{:?}` is unsupported `{}`",
                    file_type,
                    path.to_string_lossy()
                )
            }
            Self::ReadFile { path, .. } => {
                write!(f, "failed reading the file `{}`", path.to_string_lossy())
            }
            Self::ParseContents { path, .. } => write!(
                f,
                "failed to parse the contents of `{}`",
                path.to_string_lossy()
            ),
        }
    }
}
impl<E: Error> Error for ParseFileErrorKind<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::ReadSourceMetadata { source, .. } => Some(source),
            Self::ReadDirectory { source, .. } => Some(source),
            Self::ReadFile { source, .. } => Some(source),
            Self::ParseContents { source, .. } => Some(source),
            _ => None,
        }
    }
}
