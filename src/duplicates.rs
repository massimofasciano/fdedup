use crate::types::{PathData,FileSize};

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
    pub fn paths_as_display(&self) -> impl Iterator<Item=std::path::Display> + '_ {
        self.paths.iter().map(|p| p.display())
    }
    pub fn hash_as_hex(&self) -> &String {
        &self.hex_hash
    }
}

impl std::fmt::Display for Duplicates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "# {} {}",self.size(), self.hash_as_hex())?;
        for p in self.paths_as_display() {
            writeln!(f, "{}",p)?
        }
        writeln!(f)
    }
}
