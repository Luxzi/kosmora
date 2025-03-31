use core::error;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    hash::Hash,
    rc::{Rc, Weak},
    sync::RwLock,
    time::SystemTime,
};

use thiserror::Error;
use uuid::Uuid;

mod fs;
mod path;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct KosmoraFs {
    root: Rc<KosmoraINode>,
    inodes: RefCell<HashMap<Uuid, Rc<KosmoraINode>>>,
}

pub struct KosmoraFsBuilder {}
pub struct KosmoraPackageBuilder {}

impl KosmoraFsBuilder {
    fn build(self) -> KosmoraFs {
        todo!()
    }
}

impl KosmoraFs {}

pub struct KosmoraMountPoint;
pub struct KosmoraPackage {
    root: Rc<KosmoraINode>,
}

#[derive(Debug)]
pub(crate) struct KosmoraINode {
    inode_type: KosmoraINodeType,
    uuid: Uuid,
    size: u64,
    link_count: u16,
    metadata: Option<INodeMetadata>,
    content: RwLock<INodeContent>,
    parent: RefCell<Weak<KosmoraINode>>,
}

#[derive(Debug)]
enum INodeContent {
    File(Vec<u8>),
    Directory(HashMap<String, Uuid>),
}

#[derive(Debug)]
struct INodeMetadata {
    created_time: SystemTime,
    modified_time: SystemTime,
    accessed_at: SystemTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KosmoraINodeType {
    File,
    Directory,
}

impl KosmoraINodeType {
    pub const FILE: Self = KosmoraINodeType::File;
    pub const DIRECTORY: Self = KosmoraINodeType::Directory;

    fn is_file(&self) -> bool {
        if self != &Self::FILE {
            return false;
        }

        true
    }

    fn is_directory(&self) -> bool {
        !self.is_file()
    }
}

impl std::fmt::Display for KosmoraINodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KosmoraINodeType::File => "File",
                KosmoraINodeType::Directory => "Directory",
            }
        )
    }
}

#[derive(Debug, Error)]
pub struct Error {
    kind: ErrorKind,
    label: String,
    msg: Option<String>,
    source: Option<Box<Error>>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.label, self.kind)?;

        if let Some(ref details) = self.msg {
            writeln!(f, "\tDetails: {}", details)?;
        }

        if let Some(ref src) = self.source {
            writeln!(f, "\tCaused by: {}", src)?;
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
#[non_exhaustive]
enum ErrorKind {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("The path you entered was invalid.")]
    InvalidPath,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        let default_msg = kind.to_string();
        let label = match &kind {
            ErrorKind::IoError(e) => format!("{e}"),
            ErrorKind::InvalidPath => "Invalid Path".to_string(),
        };

        Self {
            kind,
            label,
            msg: Some(default_msg),
            source: None,
        }
    }

    pub(crate) fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.msg = Some(message.into());
        self
    }

    pub(crate) fn with_source(mut self, source: Error) -> Self {
        self.source = Some(Box::new(source));
        self
    }
    pub fn pretty_print(&self) {
        Self::print_error(self, 0, true);
    }

    fn print_error(err: &Error, level: usize, is_last: bool) {
        let red = "\x1b[31m";
        let blue = "\x1b[34m";
        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";

        let indent = if level > 0 {
            "   ".repeat(level - 1)
        } else {
            String::new()
        };
        let branch = if level > 0 {
            if is_last { "└── " } else { "├── " }
        } else {
            ""
        };

        println!(
            "{}{}{}{}{}: {}{}{}",
            indent, branch, red, err.label, reset, blue, err.kind, reset
        );

        // Print the details if available
        if let Some(ref details) = err.msg {
            println!(
                "{}    {}Details: {}{}{}",
                indent, "", yellow, details, reset
            );
        }

        if let Some(ref source) = err.source {
            println!("{}    Caused by:", indent);
            Self::print_error(source, level + 1, true);
        }
    }
}
