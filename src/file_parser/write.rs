use std::{
    fs::File,
    io::{self, stdout},
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use thiserror::Error;

use crate::{FileParser, FileType, write_tokens_blocking};

impl<T: TryFrom<String> + ToTokens> FileParser<T> {
    /// Write the parsed files to some target or `stdout` if None.
    pub fn write(&self, target: Option<&Path>) -> Result<(), WriteError> {
        match target {
            Some(target) => {
                if target.exists() {
                    let metadata = target
                        .metadata()
                        .map_err(|e| WriteError::Io(e, target.to_path_buf()))?;

                    let file_type = FileType::from(&metadata);
                    if file_type != FileType::File {
                        return Err(WriteError::UnsupportedTarget(
                            target.to_path_buf(),
                            file_type,
                        ));
                    }
                }

                let output_file = File::options()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(target)
                    .map_err(|e| WriteError::Io(e, target.to_path_buf()))?;

                write_tokens_blocking(self.to_token_stream(), output_file)
                    .map_err(WriteError::RustFmt)?;
            }

            None => {
                write_tokens_blocking(self.to_token_stream(), stdout())
                    .map_err(WriteError::RustFmt)?;
            }
        }

        Ok(())
    }
}

impl<T: TryFrom<String> + ToTokens> ToTokens for FileParser<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let modules = &self.modules;

        let new_tokens = quote! {
            #( #modules )*
        };
        tokens.extend(new_tokens);
    }
}

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum WriteError {
    #[error("Target '{0:?}' must be a regular file, got '{1:?}'.")]
    UnsupportedTarget(PathBuf, FileType),

    #[error("Could not write to '{1:?}': {0}")]
    Io(#[source] io::Error, PathBuf),

    #[error("rustfmt failed: {0}")]
    RustFmt(#[source] io::Error),
}
