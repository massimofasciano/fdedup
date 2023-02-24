use fdedup::HashedFiles;
use walkdir::WalkDir;

fn index_dir(hfs : &mut HashedFiles, dir : &str) {
    let walk = WalkDir::new(dir).into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());
    for entry in walk {
        hfs.add_path(entry.path().to_owned(), entry.metadata().unwrap().modified().unwrap());
    }
}

fn main() {
    let mut hfs = HashedFiles::new();
    index_dir(&mut hfs, ".");
    for dup in hfs.duplicates() {
        println!("# {} {}",dup.size(), dup.hex_hash());
        for p in dup.display_paths() {
            println!("{}",p)
        }
        println!();
    }
}
