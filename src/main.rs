use fdedup::{HashedFiles, deserialize, serialize, index_dir};

fn main() {
    let fname = "hfs.bin";
    //let mut hfs = HashedFiles::new();
    let mut hfs : HashedFiles = deserialize(fname);
    index_dir(&mut hfs, ".");
    for dup in hfs.duplicates() {
        println!("# {} {}",dup.size(), dup.hex_hash());
        for p in dup.display_paths() {
            println!("{}",p)
        }
        println!();
    }
    serialize(fname, hfs);
}
