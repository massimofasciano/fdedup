use std::{collections::HashMap, time::SystemTime};
use serde::{Serialize,Deserialize};

//const VERBOSE: bool = true;
const VERBOSE: bool = false;
macro_rules! vprintln {
    ($($x:tt)*) => { if VERBOSE { println!($($x)*); } }
}

pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    pub fn new(path : PathData, modified : SystemTime) -> GenericResult<HashedFile> {
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
        //vprintln!("add_path {:?}, {:?}", path, modified);
        if let Some(old) = self.by_path.get(&path) {
            // file is already cached
            // check last modified date and reuse if same
                if old.modified == modified {
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
            // vprintln!("{:#?}",entry);
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

