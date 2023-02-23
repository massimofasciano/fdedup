use std::{collections::HashMap};

type HashData = Vec<u8>;
type PathData = std::path::PathBuf;

#[derive(Debug)]
pub struct HashedFile {
    path : PathData,
    hash : HashData,
}

impl HashedFile {
    pub fn new(path : PathData) -> Result<HashedFile, Box<dyn std::error::Error>> {
        use sha2::{Sha512, Digest};
        use std::{io, fs};

        let mut hasher = Sha512::new();
        let mut file = fs::File::open(&path)?;
        io::copy(&mut file, &mut hasher)?;
        Ok(HashedFile{path, hash : hasher.finalize().to_vec()})
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
    pub fn add_file(&mut self, f: HashedFile) {
        if let Some(v) = self.by_hash.get_mut(&f.hash) {
            v.push(f.path.clone())
        } else {
            self.by_hash.insert(f.hash.clone(), vec!(f.path.clone()));
        };
        self.by_path.insert(f.path.clone(), f);
    }
    pub fn duplicates(& self) -> impl Iterator<Item=&Vec<PathData>> {
        self.by_hash.iter().filter_map(|(_,x)| if (*x).len() > 1 {Some(x)} else {None})
    }
    pub fn add_path(&mut self, path: PathData) {
        if let Ok(hf) = HashedFile::new(path) {
            self.add_file(hf)
        }
    }
}