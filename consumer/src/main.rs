use kosmora::{self, KosmoraINodeInteroperable};
use std::path::Path;

fn main() {
    let vfs = kosmora::KosmoraVfs::new();
    let example_path: &Path = &std::path::Path::new(".");
    example_path.to_kosmora_inode();
}
    