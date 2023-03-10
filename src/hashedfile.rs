use serde::{Serialize,Deserialize};
use std::{time::SystemTime};

use crate::types::{PathData,FileSize,HashData,Result};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct HashedFile {
    path : PathData,
    hash : HashData,
    modified : Option<SystemTime>,
    size : FileSize,
}

impl HashedFile {
    pub fn new(path : PathData, modified : Option<SystemTime>) -> Result<Self> {
        use sha2::{Sha512, Digest};
        use std::{io, fs};

        let mut hasher = Sha512::new();
        let mut file = fs::File::open(&path)?;
        let size = io::copy(&mut file, &mut hasher)?;
        Ok(Self{path, hash : hasher.finalize().to_vec(), modified, size})
    }
    pub fn path(&self) -> &PathData {
        &self.path
    }
    pub fn size(&self) -> FileSize {
        self.size
    }
    pub fn modified(&self) -> Option<SystemTime> {
        self.modified
    }
    pub fn hash(&self) -> &HashData {
        &self.hash
    }
}

impl Clone for HashedFile {
    fn clone(&self) -> Self {
        Self {path: self.path.clone(),hash : self.hash.clone(),modified : self.modified,size : self.size}
    }
}
