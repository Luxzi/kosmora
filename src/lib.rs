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