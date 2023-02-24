use fdedup::HashedFiles;
use walkdir::WalkDir;

fn main() {
    let mut hfs = HashedFiles::new();
    let walk = WalkDir::new(".").into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path());
    for entry in walk {
        hfs.add_path(entry);
    }

    for dup in hfs.duplicates() {
        println!("# {} {}",dup.size(), dup.hex_hash());
        for p in dup.display_paths() {
            println!("{}",p)
        }
        println!();
    }
}
