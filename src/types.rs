pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) type HashData = Vec<u8>;
pub(crate) type PathData = std::path::PathBuf;
pub(crate) type FileSize = u64;


