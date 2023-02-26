use serde::{Serialize,Deserialize};
use std::{time::SystemTime};

use crate::types::{PathData,FileSize,HashData,GenericResult};

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
    pub fn path(&self) -> &PathData {
        &self.path
    }
    pub fn size(&self) -> FileSize {
        self.size
    }
    pub fn modified(&self) -> SystemTime {
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
