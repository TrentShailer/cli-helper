use std::fs::Metadata;

/// The high level type of a file or directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileType {
    /// A file.
    File,

    /// A directory.
    Directory,

    /// A symbolic link.
    Symlink,

    /// Any other type.
    Unknown,
}

impl From<&Metadata> for FileType {
    fn from(value: &Metadata) -> Self {
        if value.is_dir() {
            Self::Directory
        } else if value.is_file() {
            Self::File
        } else if value.is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}
