use crate::KosmoraINodeInteroperable;

#[test]
fn create_inode() {
    let vfs = crate::KosmoraVfs::new();
    let physical_inode_path: &Path = &std::path::Path::new(".");
    physical_inode_path.to_kosmora_inode();
}