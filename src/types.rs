//pub type Result<T> = Result<T, Box<dyn std::error::Error>>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) type HashData = Vec<u8>;
pub(crate) type PathData = std::path::PathBuf;
pub(crate) type FileSize = u64;


