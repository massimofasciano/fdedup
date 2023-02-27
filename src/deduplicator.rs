use crate::types::{Result, PathData};
use crate::dedupstate::DedupState;

pub struct Deduplicator {
    dirs : Vec<PathData>,
    dedup_state : DedupState,
}

impl Deduplicator {
    pub fn new<S,D>(dirs : D) -> Self where S : Into<PathData>, D: Into<Vec<S>> {
        Self {
            dirs : dirs.into().into_iter().map(|d|d.into()).collect(),
            ..Default::default()
        }
    }
    pub fn set_threads(&mut self, threads : usize) {
        self.dedup_state.set_threads(threads);
    }
    pub fn add_dir<S>(&mut self, dir: S) where S : Into<PathData> {
        self.dirs.push(dir.into());
    }
    pub fn normalize_path(&mut self, normalize : bool) {
        self.dedup_state.set_normalize_path(normalize);
    }
    pub fn read_cache<S>(&mut self, fname: S) where S: Into<PathData> {
        let fname = fname.into();
        match self.dedup_state.read_cache(&fname) {
            Ok(_) => { }
            _ => { println!("Warning: could not load cache file {}",fname.display()); }
        }
    }
    pub fn write_cache<S>(&mut self, fname: S) -> Result<()> where S: Into<PathData> {
        self.dedup_state.write_cache(fname.into())
    }
    pub fn run(&mut self) -> Result<()> {
        for dir in &self.dirs {
            self.dedup_state.index_dir(dir)?;
        }
        for dup in self.dedup_state.duplicates() {
            println!("{}",dup);
        }
        Ok(())
    }
    pub fn set_verbosity(&mut self, verbosity : u8) {
        self.dedup_state.set_verbosity(verbosity);
    }
}

impl Default for Deduplicator {
    fn default() -> Self {
        Self {
            dirs : Vec::<PathData>::default(),
            dedup_state : DedupState::new(),
        }
    }
}
