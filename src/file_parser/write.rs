use core::{error::Error, fmt};
use std::{
    fs::File,
    io::{self, stdout},
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{FileParser, FileType, file_parser::ParseFrom, write_tokens_blocking};

impl<State, T: ParseFrom<String, State> + ToTokens> FileParser<State, T> {
    /// Write the parsed files to some target or `stdout` if None.
    pub fn write(&self, target: Option<&Path>) -> Result<(), WriteError> {
        match target {
            Some(target) => {
                if target.exists() {
                    let metadata = target.metadata().map_err(|source| WriteError {
                        kind: WriteErrorKind::ReadMetadata {
                            path: target.to_path_buf(),
                            source,
                        },
                    })?;

                    let file_type = FileType::from(&metadata);
                    if file_type != FileType::File {
                        return Err(WriteError {
                            kind: WriteErrorKind::UnsupportedFileType {
                                file_type,
                                path: target.to_path_buf(),
                            },
                        });
                    }
                }

                let output_file = File::options()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(target)
                    .map_err(|source| WriteError {
                        kind: WriteErrorKind::OpenTarget {
                            path: target.to_path_buf(),
                            source,
                        },
                    })?;

                write_tokens_blocking(self.to_token_stream(), output_file).map_err(|source| {
                    WriteError {
                        kind: WriteErrorKind::RustFmt { source },
                    }
                })?;
            }

            None => {
                write_tokens_blocking(self.to_token_stream(), stdout()).map_err(|source| {
                    WriteError {
                        kind: WriteErrorKind::RustFmt { source },
                    }
                })?;
            }
        }

        Ok(())
    }
}

impl<State, T: ParseFrom<String, State> + ToTokens> ToTokens for FileParser<State, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let modules = &self.modules;

        let new_tokens = quote! {
            #( #modules )*
        };
        tokens.extend(new_tokens);
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// Failed to write output tokens to target.
pub struct WriteError {
    /// Error variants.
    pub kind: WriteErrorKind,
}
impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to write output")
    }
}
impl Error for WriteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// Write error variants.
pub enum WriteErrorKind {
    #[non_exhaustive]
    /// The target file type is unsupported.
    UnsupportedFileType {
        /// The target file type.
        file_type: FileType,
        /// The target file path.
        path: PathBuf,
    },

    #[non_exhaustive]
    /// Opening the target for writing failed.
    OpenTarget {
        /// The target file path.
        path: PathBuf,
        /// The source IO error.
        source: io::Error,
    },

    #[non_exhaustive]
    /// Reading the target metadata failed.
    ReadMetadata {
        /// The target file path.
        path: PathBuf,
        /// The source IO error.
        source: io::Error,
    },

    #[non_exhaustive]
    /// `rustfmt` failed to be spawned or format the output.
    RustFmt {
        /// The source IO error.
        source: io::Error,
    },
}
impl fmt::Display for WriteErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnsupportedFileType { file_type, path } => write!(
                f,
                "{} exists with unsupported file type {:?}",
                path.to_string_lossy(),
                file_type
            ),
            Self::OpenTarget { path, .. } => write!(
                f,
                "failed to open/create target file {} for writing",
                path.to_string_lossy()
            ),
            Self::ReadMetadata { path, .. } => write!(
                f,
                "failed to read the metadata of the target file {}",
                path.to_string_lossy()
            ),
            Self::RustFmt { .. } => write!(f, "failed to run rustfmt on the output"),
        }
    }
}
impl Error for WriteErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::OpenTarget { source, .. } => Some(source),
            Self::ReadMetadata { source, .. } => Some(source),
            Self::RustFmt { source, .. } => Some(source),
            _ => None,
        }
    }
}
