use std::time::SystemTime;
use dashmap::DashMap;

use crate::types::{PathData,FileSize,HashData,Result};
use crate::verbose::{vprintln};
use crate::hashedfile::HashedFile;
use crate::duplicates::Duplicates;

#[derive(Debug,Default)]
pub struct DedupState {
    by_hash : DashMap<HashData,Vec<PathData>>,
    by_path : DashMap<PathData,HashedFile>,
}

impl DedupState {
    pub fn new() -> Self {
        Self::default()
    }
    pub (crate) fn add_hashed_file(&self, hf: HashedFile) {
        vprintln!(2,"adding hashed file: {}",hf.path().display());
        if let Some(mut v) = self.by_hash.get_mut(hf.hash()) {
            v.push(hf.path().clone())
        } else {
            self.by_hash.insert(hf.hash().clone(), vec!(hf.path().clone()));
        };
        self.by_path.insert(hf.path().clone(), hf);
    }
    pub (crate) fn reuse_if_cached(&self, path : &PathData, modified : &Option<SystemTime>) -> bool {
        if let Some(modified) = modified {
            if let Some(old) = self.by_path.get(path) {
                if let Some(oldmod) = old.modified() {
                    if oldmod == *modified {
                        let hf = old.clone();
                        vprintln!(2,"reusing from cache: {}",hf.path().display());
                        if let Some(mut v) = self.by_hash.get_mut(hf.hash()) {
                            v.push(hf.path().clone())
                        } else {
                            self.by_hash.insert(hf.hash().clone(), vec!(hf.path().clone()));
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
        let bhv = self.by_hash.iter()
            .filter_map(|kv| { 
                if kv.value().len() > 1 { 
                    Some(kv.value().clone()) 
                } else { 
                    None 
                } 
            }).collect::<Vec<_>>();
        let dups_iter = bhv.iter()
            .map(|v| {
                v.iter().filter_map(|p| {
                    self.by_path.get(p)
                })
            });
        for group in dups_iter {
            let group : Vec<_> = group.collect();
            if group.len() > 1 {
                let group_info = group[0].clone();
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
        // let bytes = bincode::serialize(&self.by_path.values().collect::<Vec<_>>())?;
        let bytes = bincode::serialize(&self.by_path.iter().map(|kv|kv.value().clone()).collect::<Vec<_>>())?;
        std::fs::write(fname.into(), &bytes[..])?;
        Ok(())
    }
    pub fn read_cache<S>(&mut self, fname: S) -> Result<()> where S: Into<PathData> {
        let fname = fname.into();
        let bytes = std::fs::read(fname)?;
        let cache : Vec<HashedFile> = bincode::deserialize(&bytes[..])?;
        for hf in cache.iter() {
            if !self.by_path.contains_key(hf.path()) {
                vprintln!(1,"adding to cache: {}",hf.path().display());
                self.by_path.insert(hf.path().clone(), hf.clone());
            } else {
                vprintln!(1,"aready cached: {}",hf.path().display());
            }
        }
        Ok(())
    }
}
