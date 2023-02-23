use std::{collections::HashMap};

type HashData = Vec<u8>;
type PathData = std::path::PathBuf;
type MetaData = Option<std::fs::Metadata>;
type FileSize = u64;

#[derive(Debug)]
pub struct HashedFile {
    path : PathData,
    hash : HashData,
    metadata : MetaData,
    size : FileSize,
}

impl HashedFile {
    pub fn new(path : PathData, metadata: MetaData) -> Result<HashedFile, Box<dyn std::error::Error>> {
        use sha2::{Sha512, Digest};
        use std::{io, fs};

        let mut hasher = Sha512::new();
        let mut file = fs::File::open(&path)?;
        let size = io::copy(&mut file, &mut hasher)?;
        Ok(HashedFile{path, hash : hasher.finalize().to_vec(), metadata, size})
    }
    pub fn size(&self) -> FileSize {
        self.size
    }
    pub fn metadata(&self) -> &MetaData {
        &self.metadata
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
    pub fn add_file(&mut self, f: HashedFile) {
        if let Some(v) = self.by_hash.get_mut(&f.hash) {
            v.push(f.path.clone())
        } else {
            self.by_hash.insert(f.hash.clone(), vec!(f.path.clone()));
        };
        self.by_path.insert(f.path.clone(), f);
    }
    pub fn duplicates_by_path_with_minsize(& self, minsize : FileSize) -> impl Iterator<Item=&Vec<PathData>> {
        self.by_hash.iter().filter_map(move |(_,x)| {
            if (*x).len() > 1 && 
              self.by_path.get(&(*x)[0]).unwrap().size >= minsize {
                Some(x)
            } else {
                None
            }})
    }
    pub fn duplicates(& self) -> impl Iterator<Item=impl Iterator <Item=&HashedFile>> {
        self.by_hash.iter().filter_map(|(_,x)| {
            if (*x).len() > 1 {
                Some((*x).iter().map(|p| self.by_path.get(p).unwrap()))
            } else {
                None
            }})
    }
    pub fn add_path(&mut self, path: PathData) {
        if let Ok(hf) = HashedFile::new(path,None) {
            self.add_file(hf)
        }
    }
}