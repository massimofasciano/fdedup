use fdedup::{HashedFiles, index_dir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fname = "cache.bin";
    let mut hfs = HashedFiles::new();
    hfs.read_cache(fname)?;
    index_dir(&mut hfs, ".");
    for dup in hfs.duplicates() {
        println!("{}",dup);
    }
    hfs.write_cache(fname)?;
    Ok(())
}
