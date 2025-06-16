//! # `cli-helper`
//!

mod action;
mod file_parser;
mod file_type;
mod log;
#[cfg(feature = "generate")]
mod write_tokens;

pub use action::{Action, ActionResult, State};
pub use file_parser::{FileParseError, FileParser, Module, ModuleError, to_valid_ident};
pub use file_type::FileType;
pub use log::{print_error, print_fail, print_success, print_warning};

#[cfg(feature = "generate")]
pub use file_parser::WriteError;
#[cfg(feature = "generate")]
pub use write_tokens::{write_tokens, write_tokens_blocking, write_tokens_parallel};
