use std::time::SystemTime;
use std::collections::HashMap;
#[cfg(not(feature = "mutex"))]
use std::cell::RefCell;
#[cfg(feature = "mutex")]
use std::sync::Mutex;

use crate::types::{PathData,FileSize,HashData,Result};
use crate::verbose::{vprintln};
use crate::hashedfile::HashedFile;
use crate::duplicates::Duplicates;

#[derive(Debug,Default)]
pub struct DedupState {
    #[cfg(not(feature = "mutex"))]
    by_hash : RefCell<HashMap<HashData,Vec<PathData>>>,
    #[cfg(not(feature = "mutex"))]
    by_path : RefCell<HashMap<PathData,HashedFile>>,
    #[cfg(feature = "mutex")]
    by_hash : Mutex<HashMap<HashData,Vec<PathData>>>,
    #[cfg(feature = "mutex")]
    by_path : Mutex<HashMap<PathData,HashedFile>>,
}

#[cfg(feature = "mutex")]
macro_rules! locked {
    ($data:expr) => {
        $data.lock().unwrap()
    };
}
#[cfg(not(feature = "mutex"))]
macro_rules! locked {
    ($data:expr) => {
        $data.borrow_mut()
    };
}

impl DedupState {
    pub fn new() -> Self {
        Self::default()
    }
    pub (crate) fn add_hashed_file(&self, hf: HashedFile) {
        vprintln!(2,"adding hashed file: {}",hf.path().display());
        {   // operate on by_hash inside this scope so the lock gets
            // released before operating on by_path (when using threads)
            let mut by_hash = locked!(self.by_hash); 
            if let Some(v) = by_hash.get_mut(hf.hash()) {
                v.push(hf.path().clone())
            } else {
                by_hash.insert(hf.hash().clone(), vec!(hf.path().clone()));
            };
        }
        locked!(self.by_path).insert(hf.path().clone(), hf);
    }
    pub (crate) fn reuse_if_cached(&self, path : &PathData, modified : &Option<SystemTime>) -> bool {
        if let Some(modified) = modified {
            let by_path = locked!(self.by_path);
            if let Some(old) = by_path.get(path) {
                if let Some(oldmod) = old.modified() {
                    if oldmod == *modified {
                        let hf = old.clone();
                        // when using threads it's important to drop by_path
                        // to release the lock on it after we cloned the ref
                        // to old that was pointing inside it
                        drop(by_path);
                        vprintln!(2,"reusing from cache: {}",hf.path().display());
                        let mut by_hash = locked!(self.by_hash); 
                        if let Some(v) = by_hash.get_mut(hf.hash()) {
                            v.push(hf.path().clone())
                        } else {
                            by_hash.insert(hf.hash().clone(), vec!(hf.path().clone()));
                        };
                        return true;
                    }
                }
            }
        }
        false
    }
    pub (crate) fn duplicates_with_minsize(& self, minsize : FileSize) -> Vec<Duplicates> {
        let mut result = vec!();
        let by_hash = locked!(self.by_hash);
        let by_path = locked!(self.by_path);
        let dups_iter = by_hash.iter().filter_map(|(_,x)| {
            if (*x).len() > 1 {
                Some((*x).iter().filter_map(|p| by_path.get(p)))
            } else {
                None
            }
        });
        for group in dups_iter {
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
        let bytes = bincode::serialize(&locked!(self.by_path).values().collect::<Vec<_>>())?;
        std::fs::write(fname.into(), &bytes[..])?;
        Ok(())
    }
    pub fn read_cache<S>(&mut self, fname: S) -> Result<()> where S: Into<PathData> {
        let fname = fname.into();
        let bytes = std::fs::read(fname)?;
        let cache : Vec<HashedFile> = bincode::deserialize(&bytes[..])?;
        for hf in cache.iter() {
            if !locked!(self.by_path).contains_key(hf.path()) {
                vprintln!(1,"adding to cache: {}",hf.path().display());
                locked!(self.by_path).insert(hf.path().clone(), hf.clone());
            } else {
                vprintln!(1,"already cached: {}",hf.path().display());
            }
        }
        Ok(())
    }
}
