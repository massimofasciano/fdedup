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

    // println!("{hfs:#?}");
    // println!("{:#?}",hfs.duplicates_by_path_with_minsize(1024).collect::<Vec<_>>());
    for group in hfs.duplicates() {
        let group : Vec<_> = group.collect();
        if group.len() > 1 {
            let group_info = group[0];
            if group_info.size() > 1024 {
                println!("{} {}",group_info.size(),group_info.hex_hash());
                println!("{:#?}",group.iter().map(|e| e.path()).collect::<Vec<_>>());
                println!();
            }
        }
    }
}
