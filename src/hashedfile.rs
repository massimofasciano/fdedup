use serde::{Serialize,Deserialize};
use std::{time::SystemTime};

use crate::utils::{PathData,FileSize,HashData,GenericResult};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct HashedFile {
    pub (crate) path : PathData,
    pub (crate) hash : HashData,
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
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn modified(&self) -> SystemTime {
        self.modified
    }
}

impl Clone for HashedFile {
    fn clone(&self) -> Self {
        Self {path: self.path.clone(),hash : self.hash.clone(),modified : self.modified,size : self.size}
    }
}
