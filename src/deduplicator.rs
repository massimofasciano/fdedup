use crate::utils::{GenericResult};
use crate::hashedfiles::HashedFiles;

pub struct Deduplicator {
    dirs : Vec<String>,
    hashed_files : HashedFiles,
    normalize_path : bool,
}

impl Deduplicator {
    pub fn new(dir : &str) -> Self {
        Self {
            dirs : vec!(dir.to_owned()),
            hashed_files : HashedFiles::new(),
            normalize_path : false
        }
    }
    pub fn add_dir(&mut self, dir: &str) {
        self.dirs.push(dir.to_owned());
    }
    pub fn normalize_path(&mut self, normalize : bool) {
        self.normalize_path = normalize;
    }
    pub fn read_cache(&mut self, fname: &str) {
        match self.hashed_files.read_cache(fname) {
            Ok(_) => { }
            _ => { println!("Warning: could not load cache file {}",fname); }
        }
    }
    pub fn write_cache(&mut self, fname: &str) -> GenericResult<()>{
        self.hashed_files.write_cache(fname)
    }
    pub fn run(&mut self) -> GenericResult<()> {
        for dir in &self.dirs {
            self.hashed_files.index_dir(dir,self.normalize_path)?;
        }
        for dup in self.hashed_files.duplicates() {
            println!("{}",dup);
        }
        Ok(())
    }
}

