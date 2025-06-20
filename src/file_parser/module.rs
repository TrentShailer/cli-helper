use core::{cell::LazyCell, marker::PhantomData};
use std::{
    fs,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use regex::Regex;

#[cfg(feature = "generate")]
use proc_macro2::TokenStream;
#[cfg(feature = "generate")]
use quote::{ToTokens, format_ident, quote};

use crate::{ParseFileError, ParseFileErrorKind, file_parser::ParseFrom};

/// Convert a string to a valid `ident`.
pub fn to_valid_ident(name: &str) -> String {
    let invalid_characters: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"(?:^[^a-zA-Z_]+|[^a-zA-Z0-9_]+)").unwrap());
    invalid_characters
        .replace_all(name.trim(), "_")
        .to_case(Case::Snake)
}

/// A module with a name, and parsed contents.
pub struct Module<State, T: ParseFrom<String, State>> {
    /// Parsed contents of the module.
    pub contents: T,

    /// Source file for the module.
    pub source: PathBuf,

    /// Name of the module.
    pub name: String,

    phantom_data: PhantomData<State>,
}

#[cfg(feature = "generate")]
impl<State, T: ToTokens + ParseFrom<String, State>> ToTokens for Module<State, T> {
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

impl<State, T: ParseFrom<String, State>> ParseFrom<&Path, State> for Module<State, T> {
    type Error = ParseFileError<T::Error>;

    fn parse(source: &Path, state: &mut State) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(source).map_err(|e| ParseFileError {
            kind: ParseFileErrorKind::ReadFile {
                source: e,
                path: source.to_path_buf(),
            },
        })?;

        let name = to_valid_ident(
            &source
                .file_stem()
                .expect("Path must have file")
                .to_string_lossy(),
        );

        let contents = T::parse(contents, state).map_err(|e| ParseFileError {
            kind: ParseFileErrorKind::ParseContents {
                source: e,
                path: source.to_path_buf(),
            },
        })?;

        Ok(Self {
            contents,
            source: source.to_path_buf(),
            name,
            phantom_data: Default::default(),
        })
    }
}
