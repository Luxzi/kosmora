use crate::{
    KosmoraDirectory, KosmoraFile, KosmoraFileMetadata, KosmoraINode, KosmoraINodeInteroperable,
    KosmoraINodeType,
};
use std::{boxed::Box, fs, path};
use walkdir::{DirEntry, WalkDir};

fn read_upwards(path: &path::Path) -> Option<Box<KosmoraDirectory>> {
    if let Some(parent) = path.parent() {
        return read_upwards(parent);
    }

    return None;
}

pub(crate) fn collect_physical_directory_children(path: &path::Path) -> Vec<KosmoraINode> {
    if !path.is_dir() {
        panic!("Cannot collect children of non-directory inode!");
    }

    let mut inodes: Vec<KosmoraINode> = vec![];

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let inode: KosmoraINode = match entry.path().is_file() {
            true => create_kosmora_file(
                &entry.clone().into_path(),
                Some(Box::new(
                    entry.path().parent().unwrap().to_kosmora_directory(),
                )),
            ),
            false => create_kosmora_directory(
                &entry.clone().into_path(),
                Some(Box::new(entry.path().parent().unwrap().to_kosmora_directory())),
                Some(collect_physical_directory_children(entry.path()))
            ),
        };

        inodes.push(inode);
    }

    inodes
}

pub(crate) fn create_kosmora_file(
    entry: &path::Path,
    root: Option<Box<KosmoraDirectory>>,
) -> KosmoraINode {
    let file = KosmoraFile {
        metadata: KosmoraFileMetadata {
            name: entry.file_name().unwrap().to_str().unwrap().to_string(),
            extension: None,
            size: entry.metadata().unwrap().len() as usize,
        },
        data: fs::read(entry).unwrap(),
    };

    KosmoraINode {
        inode: KosmoraINodeType::File(file),
    }
}

pub(crate) fn create_kosmora_directory(
    entry: &path::Path,
    root: Option<Box<KosmoraDirectory>>,
    content: Option<Vec<KosmoraINode>>,
) -> KosmoraINode {
    let dir = KosmoraDirectory {
        name: entry.file_name().unwrap().to_str().unwrap().to_string(),
        // parent: read_upwards(entry.path()),
        parent: root,
        children: None,
    };

    KosmoraINode {
        inode: KosmoraINodeType::Directory(dir),
    }
}
