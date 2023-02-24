use std::{collections::HashMap, time::SystemTime};
use serde::{Serialize,Deserialize};

//const VERBOSE: bool = true;
const VERBOSE: bool = false;
macro_rules! vprintln {
    ($($x:tt)*) => { if VERBOSE { println!($($x)*); } }
}

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

impl std::fmt::Display for Duplicates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "# {} {}",self.size(), self.hex_hash())?;
        for p in self.display_paths() {
            writeln!(f, "{}",p)?
        }
        writeln!(f)
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
}

impl Clone for HashedFile {
    fn clone(&self) -> Self {
        Self {path: self.path.clone(),hash : self.hash.clone(),modified : self.modified,size : self.size}
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
    fn add_file_by_hash(&mut self, f: &HashedFile) {
        if let Some(v) = self.by_hash.get_mut(&f.hash) {
            v.push(f.path.clone())
        } else {
            self.by_hash.insert(f.hash.clone(), vec!(f.path.clone()));
        };
    }
    pub fn add_path(&mut self, path: PathData, modified: SystemTime) {
        if let Some(old) = self.by_path.get(&path) {
            // file is already cached
            if old.modified == modified {
                vprintln!("reusing {}",old.path.display());
                // check last modified date and reuse if same
                self.add_file_by_hash(&old.clone());

            } else {
                // hash new entry and add it
                if let Ok(hf) = HashedFile::new(path,modified) {
                    vprintln!("hashing {}",hf.path.display());
                    self.add_file_by_hash(&hf);
                    self.by_path.insert(hf.path.clone(), hf);
                }
            }
        }
    }
    fn duplicates_as_hashed_files(& self) -> impl Iterator<Item=impl Iterator <Item=&HashedFile>> {
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
                if group_info.size > minsize {
                    result.push(Duplicates {
                        size : group_info.size,
                        hex_hash : hex::encode(&group_info.hash),
                        paths : group.iter().map(|e| e.path.clone()).collect::<Vec<_>>(),
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
    pub fn write_cache(& self, fname : &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = bincode::serialize(&self.by_path)?;
        std::fs::write(fname, &bytes[..])?;
        Ok(())
    }
    pub fn read_cache(&mut self, fname : &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = std::fs::read(fname)?;
        let cache : HashMap<PathData,HashedFile> = bincode::deserialize(&bytes[..])?;
        for (p,f) in cache.iter() {
            if !self.by_path.contains_key(p) {
                vprintln!("adding to cache: {}",p.display());
                self.by_path.insert(f.path.clone(), f.clone());
            } else {
                vprintln!("aready cached: {}",p.display());
            }
        }
        Ok(())
    }
}

pub fn index_dir(hfs : &mut HashedFiles, dir : &str) -> Result<(), Box<dyn std::error::Error>> {
    let walk = walkdir::WalkDir::new(dir).into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());
    for entry in walk {
        hfs.add_path(entry.path().to_owned(), entry.metadata()?.modified()?);
    }
    Ok(())
}

