use serde::{Serialize,Deserialize};
use std::{collections::HashMap, time::SystemTime};

use crate::utils::{PathData,FileSize,HashData,GenericResult,vprintln,vvprintln};
use crate::hashedfile::HashedFile;
use crate::duplicates::Duplicates;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct HashedFiles {
    by_hash : HashMap<HashData,Vec<PathData>>,
    by_path : HashMap<PathData,HashedFile>,
}

impl HashedFiles {
    pub fn new() -> HashedFiles {
        HashedFiles {
            by_hash : HashMap::new(),
            by_path : HashMap::new(),
        }
    }
    fn add_file_by_hash(&mut self, f: &HashedFile) {
        if let Some(v) = self.by_hash.get_mut(f.hash()) {
            v.push(f.path.clone())
        } else {
            self.by_hash.insert(f.hash().clone(), vec!(f.path.clone()));
        };
    }
    pub fn add_path(&mut self, path: PathData, modified: SystemTime) {
        vvprintln!("add_path {:?}, {:?}", path, modified);
        if let Some(old) = self.by_path.get(&path) {
            // file is already cached
            // check last modified date and reuse if same
                if old.modified() == modified {
                vprintln!("reusing {}",old.path.display());
                self.add_file_by_hash(&old.clone());
                return;
            }
        }
        // hash new entry and add it
        if let Ok(hf) = HashedFile::new(path,modified) {
            vprintln!("hashing {}",hf.path.display());
            self.add_file_by_hash(&hf);
            self.by_path.insert(hf.path.clone(), hf);
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
                        group.iter().map(|e| e.path.clone()).collect::<Vec<_>>(),
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
    pub fn write_cache(& self, fname : &str) -> GenericResult<()> {
        let bytes = bincode::serialize(&self.by_path.values().collect::<Vec<_>>())?;
        std::fs::write(fname, &bytes[..])?;
        Ok(())
    }
    pub fn read_cache(&mut self, fname : &str) -> GenericResult<()> {
        let bytes = std::fs::read(fname)?;
        let cache : Vec<HashedFile> = bincode::deserialize(&bytes[..])?;
        for f in cache.iter() {
            if !self.by_path.contains_key(&f.path) {
                vprintln!("adding to cache: {}",f.path.display());
                self.by_path.insert(f.path.clone(), f.clone());
            } else {
                vprintln!("aready cached: {}",f.path.display());
            }
        }
        Ok(())
    }
    pub fn index_dir(&mut self, dir : &str, normalize_path : bool) -> GenericResult<()> {
        let walk = walkdir::WalkDir::new(dir).into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());
        for entry in walk {
            vvprintln!("{:#?}",entry);
            use std::path::PathBuf;
            let mut path = entry.path().to_owned();
            if normalize_path && std::path::MAIN_SEPARATOR != '/' {
                // if normalize_path and the OS path separator is not '/' try to convert to that
                if let Some(s) = path.to_str() {
                    path = PathBuf::from(s.replace(std::path::MAIN_SEPARATOR, "/"));
                }
            }
            self.add_path(path, entry.metadata()?.modified()?);
        }
        Ok(())
    }
}
