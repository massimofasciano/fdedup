use std::{collections::HashMap, time::SystemTime};
use serde::{Serialize,Deserialize};

type HashData = Vec<u8>;
type PathData = std::path::PathBuf;
type FileSize = u64;

#[derive(Debug)]
pub struct Duplicates {
    paths : Vec<PathData>,
    hex_hash : String,
    size : FileSize,
}

impl Duplicates {
    pub fn new(paths : Vec<PathData>, hex_hash : String, size : FileSize) -> Duplicates {
        Duplicates { paths, hex_hash, size }
    }
    pub fn size(&self) -> FileSize {
        self.size
    }
    pub fn paths(&self) -> &Vec<PathData> {
        &self.paths
    }
    pub fn display_paths(&self) -> impl Iterator<Item=std::path::Display> + '_ {
        self.paths.iter().map(|p| p.display())
    }
    pub fn hex_hash(&self) -> &String {
        &self.hex_hash
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct HashedFile {
    path : PathData,
    hash : HashData,
    modified : SystemTime,
    size : FileSize,
}

impl HashedFile {
    pub fn new(path : PathData, modified : SystemTime) -> Result<HashedFile, Box<dyn std::error::Error>> {
        use sha2::{Sha512, Digest};
        use std::{io, fs};

        let mut hasher = Sha512::new();
        let mut file = fs::File::open(&path)?;
        let size = io::copy(&mut file, &mut hasher)?;
        Ok(HashedFile{path, hash : hasher.finalize().to_vec(), modified, size})
    }
    pub fn size(&self) -> FileSize {
        self.size
    }
    pub fn modified(&self) -> &SystemTime {
        &self.modified
    }
    pub fn path(&self) -> &PathData {
        &self.path
    }
    pub fn hash(&self) -> &HashData {
        &self.hash
    }
    pub fn hex_hash(&self) -> String {
        hex::encode(&self.hash)
    }
}

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
    pub fn get_by_path (&self, path : &PathData) -> Option<&HashedFile> {
        self.by_path.get(path)
    }
    fn add_file(&mut self, f: HashedFile) {
        if let Some(v) = self.by_hash.get_mut(&f.hash) {
            v.push(f.path.clone())
        } else {
            self.by_hash.insert(f.hash.clone(), vec!(f.path.clone()));
        };
        self.by_path.insert(f.path.clone(), f);
    }
    pub fn add_path(&mut self, path: PathData, modified: SystemTime) {
        if let Some(old) = self.get_by_path(&path) {
            // file is already indexed
            if old.modified == modified {
                // check last modified date
                return;
            } else {
                // invalidate existing data
                if let Some(old_dups) = self.by_hash.get(&old.hash) {
                    let mut new_dups = old_dups.clone();
                    new_dups.remove(new_dups.iter().position(|x| *x == path).unwrap());
                    self.by_hash.insert(old.hash.clone(), new_dups);
                }
            }
        }
        if let Ok(hf) = HashedFile::new(path,modified) {
            self.add_file(hf)
        }
    }
    pub fn duplicates_as_hashed_files(& self) -> impl Iterator<Item=impl Iterator <Item=&HashedFile>> {
        self.by_hash.iter().filter_map(|(_,x)| {
            if (*x).len() > 1 {
                Some((*x).iter().map(|p| self.by_path.get(p).unwrap()))
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
                    result.push(Duplicates {
                        size : group_info.size(),
                        hex_hash : group_info.hex_hash(),
                        paths : group.iter().map(|e| e.path().clone()).collect::<Vec<_>>(),
                    })
                }
            }
        }
        result.sort_by(|a, b| a.size.cmp(&b.size));
        result
    }
    pub fn duplicates(& self) -> Vec<Duplicates> {
        self.duplicates_with_minsize(0)
    }
}