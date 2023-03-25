pub type Result<T> = anyhow::Result<T>;

pub(crate) type HashData = Vec<u8>;
pub(crate) type PathData = std::path::PathBuf;
pub(crate) type FileSize = u64;
