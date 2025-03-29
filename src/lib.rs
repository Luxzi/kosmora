use std::{boxed::Box, fs, io::Seek, path, vec};
use tree::{collect_physical_directory_children, create_kosmora_directory, create_kosmora_file};
use walkdir::{DirEntry, WalkDir};

mod tree;

#[derive(Debug)]
pub struct KosmoraVfsBuilder {
    index: KosmoraIndex,
    packages: Vec<KosmoraPackage>,
}

#[derive(Debug)]
pub struct KosmoraVfs {
    builder: KosmoraVfsBuilder
}

#[derive(Debug)]
struct KosmoraIndex {
    index: Vec<KosmoraPackage>,
}

#[derive(Debug)]
struct KosmoraPackage {
    id: usize,
    inode_index: KosmoraDirectory,
}

#[derive(Debug, Clone)]
pub(crate) enum KosmoraINodeType {
    File(KosmoraFile),
    Directory(KosmoraDirectory),
}

#[derive(Debug, Clone)]
pub(crate) struct KosmoraFileMetadata {
    name: String,
    extension: Option<String>,
    size: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct KosmoraFile {
    metadata: KosmoraFileMetadata,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct KosmoraDirectory {
    name: String,
    parent: Option<Box<KosmoraDirectory>>,
    children: Option<Vec<KosmoraINode>>,
}

#[derive(Debug, Clone)]
pub(crate) struct KosmoraINode {
    inode: KosmoraINodeType,
}

impl KosmoraVfsBuilder {
    pub fn new() -> Self {
        KosmoraVfsBuilder {
            index: KosmoraIndex { index: vec![] },
            packages: vec![],
        }
    }

    pub fn new_directory<T: KosmoraINodeInteroperable>(&mut self, virtual_path: T) -> Self {
        todo!()
    }

    pub fn add_directory<T: KosmoraINodeInteroperable>(&mut self, physical_path: T, virtual_path: T) -> Self {
        todo!()
    }

    pub fn add_file<T: KosmoraINodeInteroperable>(&mut self, physical_path: T, virtual_path: T) -> Self {
        todo!()
    }

    pub fn build(&mut self) {
        todo!()
    }
}

pub(crate) trait KosmoraINodeInteroperable {
    fn collect_directory_children(&self) -> Vec<KosmoraINode>;
    fn to_kosmora_inode(&self) -> KosmoraINode;
    fn to_kosmora_directory(&self) -> KosmoraDirectory;
    fn to_kosmora_file(&self) -> KosmoraFile;
}

impl KosmoraINodeInteroperable for std::path::Path {
    fn collect_directory_children(&self) -> Vec<KosmoraINode> {
        tree::collect_physical_directory_children(self)
    }

    fn to_kosmora_inode(&self) -> KosmoraINode {
        if !self.exists() {
            panic!("Path does not exist");
        }

        if self.is_dir() {
            return create_kosmora_directory(self, None, Some(collect_physical_directory_children(self)));
        }

        if self.is_file() {
            return create_kosmora_file(self, Some(Box::new(self.parent().unwrap().to_kosmora_directory())));
        }
        
        panic!("Unsupported path type");
    }

    fn to_kosmora_directory(&self) -> KosmoraDirectory {
        if !self.is_dir() {
            panic!("Cannot convert file inode into Kosmora directory inode!");
        }

        KosmoraDirectory {
            name: self.file_name().unwrap().to_string_lossy().into(),
            parent: None,
            children: None,
        }
    }

    fn to_kosmora_file(&self) -> KosmoraFile {
        if !self.is_file() {
            panic!("Cannot convert directory inode into Kosmora file inode!")
        }

        KosmoraFile {
            metadata: KosmoraFileMetadata {
                name: self.file_name().unwrap().to_string_lossy().into(),
                extension: Some(
                    self.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split(".")
                        .last()
                        .unwrap()
                        .into(),
                ),
                size: self.metadata().unwrap().len() as usize,
            },
            data: fs::read(self).unwrap(),
        }
    }
}

impl KosmoraDirectory {
    fn with_children(&self, children: Vec<KosmoraINode>) -> KosmoraDirectory {
        KosmoraDirectory {
            name: self.name.clone(),
            parent: self.parent.clone(),
            children: Some(children),
        }
    }
}
