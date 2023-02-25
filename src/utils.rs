pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) type HashData = Vec<u8>;
pub(crate) type PathData = std::path::PathBuf;
pub(crate) type FileSize = u64;

//pub(crate) const VERBOSE: bool = true;
pub(crate) const VERBOSE: bool = false;
macro_rules! vprintln {
    ($($x:tt)*) => { if crate::utils::VERBOSE { println!($($x)*); } }
}
pub(crate) use vprintln; 
