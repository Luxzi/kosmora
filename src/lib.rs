use std::{boxed::Box, fs, io::Seek, path, vec};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct KosmoraVfs {
    index: KosmoraIndex,
    packages: Vec<KosmoraPackage>,
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

#[derive(Debug)]
pub enum KosmoraINodeType {
    File(KosmoraFile),
    Directory(KosmoraDirectory),
}

#[derive(Debug)]
pub struct KosmoraFileMetadata {
    name: String,
    extension: Option<String>,
    size: usize,
}

#[derive(Debug)]
pub struct KosmoraFile {
    metadata: KosmoraFileMetadata,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct KosmoraDirectory {
    name: String,
    parent: Option<Box<KosmoraDirectory>>,
    children: Option<Vec<KosmoraINode>>,
}

#[derive(Debug)]
pub struct KosmoraINode {
    inode: KosmoraINodeType,
}

impl KosmoraVfs {
    pub fn new() -> Self {
        KosmoraVfs {
            index: KosmoraIndex { index: Vec::new() },
            packages: Vec::new(),
        }
    }

    pub fn create_directory(virtual_path: &str) {}
    pub fn add_directory(real_path: &str, virtual_path: &str) {}
    pub fn add_file(real_path: &str, virtual_path: &str) {}
}

pub trait KosmoraINodeInteroperable {
    fn collect_directory_children(&self) -> Vec<KosmoraINode>;
    fn to_kosmora_inode(&self) -> KosmoraINode;
}

impl KosmoraINodeInteroperable for std::path::Path {
    fn collect_directory_children(&self) -> Vec<KosmoraINode> {
        if !self.is_dir() {}
        dbg!(self);

        let mut inodes: Vec<KosmoraINode> = vec![];

        for entry in WalkDir::new(&self).into_iter().filter_map(|e| e.ok()) {
            // println!("{}", entry.path().display());
            let inode: KosmoraINode = match entry.path().is_file() {
                true => {
                    let file = KosmoraFile {
                        metadata: KosmoraFileMetadata {
                            name: entry.file_name().to_string_lossy().into(),
                            extension: None,
                            size: entry.path().metadata().unwrap().len() as usize,
                        },
                        data: fs::read(entry.path()).unwrap(),
                    };
                    KosmoraINode {
                        inode: KosmoraINodeType::File(file),
                    }
                }
                false => {
                    fn read_upwards(path: &path::Path) -> Option<Box<KosmoraDirectory>> { 

                        if let Some(parent) = path.parent() {
                            println!("{parent:#?}");
                            return read_upwards(parent)
                        } else {
                            return None
                        }
                    }
                    let dir = KosmoraDirectory {
                        name: entry.file_name().to_string_lossy().into(),
                        parent: read_upwards(entry.path()),
                        children: None,
                    };
                    KosmoraINode {
                        inode: KosmoraINodeType::Directory(dir),
                    }
                }
            };

            inodes.push(inode);
        }
        
        inodes
    }

    fn to_kosmora_inode(&self) -> KosmoraINode {
        if !self.exists() {
            panic!("Path does not exist");
        }

        if self.is_dir() {
            let dir = KosmoraDirectory {
                name: self
                    .components()
                    .last()
                    .unwrap()
                    .as_os_str()
                    .to_string_lossy()
                    .into(),
                parent: None,
                children: Some(self.collect_directory_children()),
            };
            return KosmoraINode {
                inode: KosmoraINodeType::Directory(dir),
            };
        }

        if self.is_file() {
            let meta = self
                .metadata()
                .map_err(|_| panic!("Failed to get metadata"))
                .ok()
                .expect("Failed to get metadata");

            let file_metadata = KosmoraFileMetadata {
                name: String::from(self.file_name().unwrap().to_str().unwrap().to_string()),
                extension: Some(
                    self.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split(".")
                        .last()
                        .unwrap()
                        .to_string(),
                ),
                size: meta.len() as usize,
            };

            let file = KosmoraFile {
                metadata: file_metadata,
                data: std::fs::read(self).unwrap(),
            };

            return KosmoraINode {
                inode: KosmoraINodeType::File(file),
            };
        }
        panic!("Unsupported path type");
    }
}
