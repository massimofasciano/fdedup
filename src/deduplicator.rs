use crate::types::{GenericResult};
use crate::dedupstate::DedupState;

pub struct Deduplicator {
    dirs : Vec<String>,
    dedup_state : DedupState,
    normalize_path : bool,
}

impl Deduplicator {
    pub fn new(dirs : &[&str]) -> Self {
        Self {
            dirs : dirs.iter().map(|&s|s.to_owned()).collect(),
            ..Default::default()
        }
    }
    pub fn add_dir(&mut self, dir: &str) {
        self.dirs.push(dir.to_owned());
    }
    pub fn normalize_path(&mut self, normalize : bool) {
        self.normalize_path = normalize;
    }
    pub fn read_cache(&mut self, fname: &str) {
        match self.dedup_state.read_cache(fname) {
            Ok(_) => { }
            _ => { println!("Warning: could not load cache file {}",fname); }
        }
    }
    pub fn write_cache(&mut self, fname: &str) -> GenericResult<()>{
        self.dedup_state.write_cache(fname)
    }
    pub fn run(&mut self) -> GenericResult<()> {
        for dir in &self.dirs {
            self.dedup_state.index_dir(dir,self.normalize_path)?;
        }
        for dup in self.dedup_state.duplicates() {
            println!("{}",dup);
        }
        Ok(())
    }
}

impl Default for Deduplicator {
    fn default() -> Self {
        Self {
            dirs : Vec::<String>::default(),
            dedup_state : DedupState::new(),
            normalize_path : false
        }
    }
}
