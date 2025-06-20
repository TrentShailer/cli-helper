use core::{cell::LazyCell, error::Error, fmt, marker::PhantomData};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use regex::Regex;

#[cfg(feature = "generate")]
use proc_macro2::TokenStream;
#[cfg(feature = "generate")]
use quote::{ToTokens, format_ident, quote};

use crate::file_parser::ParseFrom;

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
    type Error = ParseModuleError<T::Error>;

    fn parse(source: &Path, state: &mut State) -> Result<Self, Self::Error> {
        let contents = fs::read_to_string(source).map_err(|e| ParseModuleError {
            path: source.to_path_buf(),
            kind: ParseModuleErrorKind::ReadFile { source: e },
        })?;

        let name = to_valid_ident(
            &source
                .file_stem()
                .expect("Path must have file")
                .to_string_lossy(),
        );

        let contents = T::parse(contents, state).map_err(|e| ParseModuleError {
            path: source.to_path_buf(),
            kind: ParseModuleErrorKind::ParseContents { source: e },
        })?;

        Ok(Self {
            contents,
            source: source.to_path_buf(),
            name,
            phantom_data: Default::default(),
        })
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// Failed to parse the module.
pub struct ParseModuleError<E: Error + 'static> {
    /// Path to the source file.
    pub path: PathBuf,
    /// The error variants.
    pub kind: ParseModuleErrorKind<E>,
}
impl<E: Error> fmt::Display for ParseModuleError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse module {}", self.path.to_string_lossy())
    }
}
impl<E: Error> Error for ParseModuleError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// Parse module error variants.
pub enum ParseModuleErrorKind<E: Error + 'static> {
    #[non_exhaustive]
    /// Failed to read the source file.
    ReadFile {
        /// The source IO error.
        source: io::Error,
    },

    #[non_exhaustive]
    /// Failed to parse the contents of the source file.
    ParseContents {
        /// The source parse error.
        source: E,
    },
}
impl<E: Error> fmt::Display for ParseModuleErrorKind<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ReadFile { .. } => write!(f, "failed to read module file"),
            Self::ParseContents { .. } => write!(f, "file to parse module contents"),
        }
    }
}
impl<E: Error> Error for ParseModuleErrorKind<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::ReadFile { source } => Some(source),
            Self::ParseContents { source } => Some(source),
        }
    }
}
