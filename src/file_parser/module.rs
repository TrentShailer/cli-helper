use core::cell::LazyCell;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use regex::Regex;
use thiserror::Error;

/// Convert a string to a valid `ident`.
pub fn to_valid_ident(name: &str) -> String {
    let invalid_characters: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"(?:^[^a-zA-Z_]+|[^a-zA-Z0-9_]+)").unwrap());
    invalid_characters
        .replace_all(name.trim(), "_")
        .to_case(Case::Snake)
}

/// A module with a name, and parsed contents.
pub struct Module<T: TryFrom<String>> {
    /// Parsed contents of the module.
    pub contents: T,

    /// Source file for the module.
    pub source: PathBuf,

    /// Name of the module.
    pub name: String,
}

impl<T: ToTokens + TryFrom<String>> ToTokens for Module<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let contents = &self.contents;

        let new_tokens = quote! {
            pub mod #name {
                #contents
            }
        };

        tokens.extend(new_tokens);
    }
}

impl<T: TryFrom<String>> TryFrom<&Path> for Module<T> {
    type Error = ModuleError<T::Error>;

    fn try_from(source: &Path) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(source)?;

        let name = to_valid_ident(
            &source
                .file_stem()
                .expect("Path must have file")
                .to_string_lossy(),
        );

        let contents = T::try_from(contents).map_err(ModuleError::ParseContents)?;

        Ok(Self {
            contents,
            source: source.to_path_buf(),
            name,
        })
    }
}

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum ModuleError<E> {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("{0}")]
    ParseContents(#[source] E),
}
