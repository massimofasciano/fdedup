use crate::types::{Result, PathData};
use crate::dedupstate::DedupState;
use crate::duplicates::Duplicates;
use crate::hashedfile::HashedFile;

pub struct Deduplicator {
    dirs : Vec<PathData>,
    dedup_state : DedupState,
    normalize_path : bool,
    verbosity : u8,
    threads : usize,
}

impl Deduplicator {
    pub fn new<S,D>(dirs : D) -> Self where S : Into<PathData>, D: Into<Vec<S>> {
        Self {
            dirs : dirs.into().into_iter().map(|d|d.into()).collect(),
            ..Default::default()
        }
    }
    pub fn set_threads(&mut self, threads : usize) {
        self.threads = threads;
    }
    pub fn add_dir<S>(&mut self, dir: S) where S : Into<PathData> {
        self.dirs.push(dir.into());
    }
    pub fn set_normalize_path(&mut self, normalize : bool) {
        self.normalize_path = normalize;
    }
    pub fn set_verbosity(&mut self, verbosity : u8) {
        self.verbosity = verbosity;
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
    #[cfg(all(feature = "rayon", feature = "threads"))]
    pub fn run(&self) -> Result<Vec<Duplicates>> {
        rayon::scope(|s| {
            for dir in &self.dirs {
                let walk = walkdir::WalkDir::new(dir).into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file());
                for entry in walk {
                    let mut path = entry.path().to_owned();
                    if self.normalize_path {
                        apply_path_normalization(&mut path);
                    }
                    //let modified = entry.metadata()?.modified()?; //deal with these 2 possible errors ?
                    let modified = entry.metadata().unwrap().modified().unwrap();
                    s.spawn(move |_| {
                        if !self.dedup_state.reuse_if_cached(&path, &modified) {
                            if let Ok(hf) = HashedFile::new(path,modified) {
                                self.dedup_state.add_hashed_file(hf);
                            }
                        }
                    });
                }
            }
        });
        Ok(self.dedup_state.duplicates())
    }
    #[cfg(not(feature = "threads"))]
    pub fn run(&mut self) -> Result<Vec<Duplicates>> {
        let state = &mut self.dedup_state;
        for dir in &self.dirs {
            let walk = walkdir::WalkDir::new(dir).into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());
            for entry in walk {
                let mut path = entry.path().to_owned();
                if self.normalize_path {
                    apply_path_normalization(&mut path);
                }
                let modified = entry.metadata()?.modified()?;
                if !state.reuse_if_cached(&path, &modified) {
                    if let Ok(hf) = HashedFile::new(path,modified) {
                        state.add_hashed_file(hf);
                    }
                }
            }
        }
        Ok(self.dedup_state.duplicates())
    }
    #[cfg(all(feature = "threadpool", feature = "threads"))]
    pub fn run(&mut self) -> Result<Vec<Duplicates>> {
        use threadpool::ThreadPool;
        use std::sync::mpsc::channel;
        use std::cmp::max;
        let (tx, rx) = channel();
        let pool = ThreadPool::new(max(self.threads as usize,1));
        for dir in &self.dirs {
            let walk = walkdir::WalkDir::new(dir).into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file());
            for entry in walk {
                let mut path = entry.path().to_owned();
                if self.normalize_path {
                    apply_path_normalization(&mut path);
                }
                let modified = entry.metadata()?.modified()?;
                if !self.dedup_state.reuse_if_cached(&path, &modified) {
                    let txc = tx.clone();
                    pool.execute(move|| {
                        if let Ok(hf) = HashedFile::new(path,modified) {
                            txc.send(Some(hf)).unwrap();
                        } else {
                            txc.send(None).unwrap();
                        }
                    });
                }
            }
        }
        drop(tx);
        for hfo in rx {
            if let Some(hf) = hfo {
                self.dedup_state.add_hashed_file(hf);
            }
        }
        Ok(self.dedup_state.duplicates())
    }
}

impl Default for Deduplicator {
    fn default() -> Self {
        Self {
            dirs : Vec::<PathData>::default(),
            dedup_state : DedupState::new(),
            normalize_path : false,
            verbosity : 0,
            threads : 1,
        }
    }
}

fn apply_path_normalization(path: &mut PathData) {
    if std::path::MAIN_SEPARATOR != '/' {
        // if normalize_path and the OS path separator is not '/' try to convert to that
        if let Some(s) = path.to_str() {
            *path = PathData::from(s.replace(std::path::MAIN_SEPARATOR, "/"));
        }
    }
}

