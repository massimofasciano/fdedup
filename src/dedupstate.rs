use serde::{Serialize,Deserialize};
use std::{collections::HashMap, time::SystemTime};

use crate::types::{PathData,FileSize,HashData,Result};
use crate::verbose::{vprintln};
use crate::hashedfile::HashedFile;
use crate::duplicates::Duplicates;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct DedupState {
    by_hash : HashMap<HashData,Vec<PathData>>,
    by_path : HashMap<PathData,HashedFile>,
    verbosity : u8,
    threads : usize,
    normalize_path: bool,
}

impl DedupState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_threads(&mut self, threads : usize) {
        self.threads = threads;
    }
    pub fn set_normalize_path(&mut self, normalize_path : bool) {
        self.normalize_path = normalize_path;
    }
    fn index_file_by_hash(&mut self, f: &HashedFile) {
        if let Some(v) = self.by_hash.get_mut(f.hash()) {
            v.push(f.path().clone())
        } else {
            self.by_hash.insert(f.hash().clone(), vec!(f.path().clone()));
        };
    }
    fn add_hashed_file(&mut self, hf: HashedFile) {
        vprintln!(3,self.verbosity,"add_hashed_file {:?}", hf);
        self.index_file_by_hash(&hf);
        self.by_path.insert(hf.path().clone(), hf);
    }
    fn reuse_if_cached(&mut self, path : &PathData, modified : &SystemTime) -> bool {
        if let Some(old) = self.by_path.get(path) {
            if old.modified() == *modified {
                vprintln!(1,self.verbosity,"cache hit for {}",old.path().display());
                self.add_hashed_file(old.clone());
                return true;
            }
        }
        false
    }
    fn apply_path_normalization(&self, path: &mut PathData) {
        if self.normalize_path && std::path::MAIN_SEPARATOR != '/' {
            // if normalize_path and the OS path separator is not '/' try to convert to that
            if let Some(s) = path.to_str() {
                *path = PathData::from(s.replace(std::path::MAIN_SEPARATOR, "/"));
            }
        }
    }
    fn duplicates_as_hashed_files(& self) -> impl Iterator<Item=impl Iterator <Item=&HashedFile>> {
        self.by_hash.iter().filter_map(|(_,x)| {
            if (*x).len() > 1 {
                Some((*x).iter().filter_map(|p| self.by_path.get(p)))
            } else {
                None
            }})
    }
    pub fn duplicates_with_minsize(& self, minsize : FileSize) -> Vec<Duplicates> {
        let mut result = vec!();
        for group in self.duplicates_as_hashed_files() {
            let group : Vec<_> = group.collect();
            if group.len() > 1 {
                let group_info = group[0];
                if group_info.size() > minsize {
                    result.push(Duplicates::new(
                        group.iter().map(|e| e.path().clone()).collect::<Vec<_>>(),
                        hex::encode(group_info.hash()),
                        group_info.size()
                    ))
                }
            }
        }
        result.sort_by(|a, b| a.size().cmp(&b.size()));
        result
    }
    pub fn duplicates(& self) -> Vec<Duplicates> {
        self.duplicates_with_minsize(0)
    }
    pub fn write_cache<S>(&mut self, fname: S) -> Result<()> where S: Into<PathData> {
        let bytes = bincode::serialize(&self.by_path.values().collect::<Vec<_>>())?;
        std::fs::write(fname.into(), &bytes[..])?;
        Ok(())
    }
    pub fn read_cache<S>(&mut self, fname: S) -> Result<()> where S: Into<PathData> {
        let fname = fname.into();
        let bytes = std::fs::read(fname)?;
        let cache : Vec<HashedFile> = bincode::deserialize(&bytes[..])?;
        for f in cache.iter() {
            if !self.by_path.contains_key(f.path()) {
                vprintln!(1,self.verbosity,"adding to cache: {}",f.path().display());
                self.by_path.insert(f.path().clone(), f.clone());
            } else {
                vprintln!(1,self.verbosity,"aready cached: {}",f.path().display());
            }
        }
        Ok(())
    }
    #[cfg(feature = "threadpool")]
    pub fn index_dir_multi_threaded<S>(&mut self, dir : S) -> Result<()> where S : Into<PathData> {
        use threadpool::ThreadPool;
        use std::sync::mpsc::channel;
        use std::cmp::max;
        let (tx, rx) = channel();
        let pool = ThreadPool::new(max(self.threads as usize,1));
        let walk = walkdir::WalkDir::new(dir.into()).into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());
        for entry in walk {
            vprintln!(4,self.verbosity,"{:#?}",entry);
            let mut path = entry.path().to_owned();
            self.apply_path_normalization(&mut path);
            let modified = entry.metadata()?.modified()?;
            if !self.reuse_if_cached(&path, &modified) {
                let txc = tx.clone();
                #[cfg(feature = "verbose")]
                let verbosity = self.verbosity;
                pool.execute(move|| {
                    vprintln!(1,verbosity,"start hashing job for {}",path.display());
                    if let Ok(hf) = HashedFile::new(path,modified) {
                        vprintln!(1,verbosity,"end hashing job for {}",hf.path().display());
                        txc.send(Some(hf)).unwrap();
                    } else {
                        txc.send(None).unwrap();
                    }
                });
            }
        }
        drop(tx);
        for hfo in rx {
            if let Some(hf) = hfo {
                self.add_hashed_file(hf);
            }
        }
        Ok(())
    }
    pub fn index_dir_single_threaded<S>(&mut self, dir : S) -> Result<()> where S : Into<PathData> {
        let walk = walkdir::WalkDir::new(dir.into()).into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());
        for entry in walk {
            vprintln!(4,self.verbosity,"{:#?}",entry);
            let mut path = entry.path().to_owned();
            self.apply_path_normalization(&mut path);
            let modified = entry.metadata()?.modified()?;
            if !self.reuse_if_cached(&path, &modified) {
                vprintln!(1,self.verbosity,"start hashing {}",path.display());
                if let Ok(hf) = HashedFile::new(path,modified) {
                    vprintln!(1,self.verbosity,"end hashing {}",hf.path().display());
                    self.add_hashed_file(hf);
                }
            }
        }
        Ok(())
    }
    pub fn index_dir<S>(&mut self, dir : S) -> Result<()> where S : Into<PathData> {
        #[cfg(feature = "threads")]
        return self.index_dir_multi_threaded(dir);
        #[cfg(not(feature = "threads"))]
        return self.index_dir_single_threaded(dir);
    }
    pub fn set_verbosity(&mut self, verbosity : u8) {
        self.verbosity = verbosity;
    }
}
