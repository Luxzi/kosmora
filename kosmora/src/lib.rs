use std::boxed::Box;

pub struct KosmoraVfs {
    index: KosmoraIndex,
    packages: Vec<KosmoraPackage>   
}

struct KosmoraIndex {
    index: Vec<KosmoraPackage>
}
struct KosmoraPackage {
    id: usize,
    inode_index: KosmoraDirectory 
}

pub enum KosmoraINodeType {
    File(KosmoraFile),
    Directory(KosmoraDirectory)
}

pub struct KosmoraFileMetadata {
    name: String,
    extension: Option<String>,
    size: usize,
}

pub struct KosmoraFile {
    metadata: KosmoraFileMetadata,
    data: Vec<u8>
}

pub struct KosmoraDirectory {
    name: String,
    parent: Option<Box<KosmoraDirectory>>,
    children: Option<Box<KosmoraINode>>
}

pub struct KosmoraINode {
    inode: KosmoraINodeType
}

impl KosmoraVfs {
    pub fn new() -> Self {
        KosmoraVfs {
            index: KosmoraIndex { index: Vec::new() },
            packages: Vec::new()
        }
    }

    pub fn create_directory(virtual_path: &str) {}
    pub fn add_directory(real_path: &str, virtual_path: &str) {}
    pub fn add_file(real_path: &str, virtual_path: &str) {}
}

pub trait KosmoraINodeInteroperable {
    fn collect_directory_children(&self) -> KosmoraINode;
    fn to_kosmora_inode(&self) -> KosmoraINode;
}

impl KosmoraINodeInteroperable for std::path::Path {
    fn collect_directory_children(&self) -> KosmoraINode {
        if !self.exists() || !self.is_dir() {
            panic!("nuh uh that isnt allowed")
        }
        
        dbg!(self);
        match std::fs::File::open(&self) {
            Ok(file) => {
                todo!()
            },
            Err(e) => {
                eprintln!("{e}");
                todo!()
            }
        }
    }
    
    fn to_kosmora_inode(&self) -> KosmoraINode {
        if !self.exists() {
            panic!("Path does not exist");
        }

        if self.is_dir() {
            let dir = KosmoraDirectory {
                name: self.components().last().unwrap().as_os_str().to_string_lossy().into(),
                parent: None,
                children: Some(Box::new(self.collect_directory_children())),
            };
            return KosmoraINode { inode: KosmoraINodeType::Directory(dir) };
        } 

        if self.is_file() {
            let meta = self.metadata()
                .map_err(|_| panic!("Failed to get metadata"))
                .ok()
                .expect("Failed to get metadata");

            let file_metadata = KosmoraFileMetadata {
                name: String::from(self.file_name().unwrap().to_str().unwrap().to_string()),
                extension: Some(self.file_name().unwrap().to_str().unwrap().split(".").last().unwrap().to_string()),
                size: meta.len() as usize,
            };
            
            let file = KosmoraFile {
                metadata: file_metadata,
                data: std::fs::read(self).unwrap(),
            };

            return KosmoraINode { inode: KosmoraINodeType::File(file) }
        }
        panic!("Unsupported path type");
    }
}