pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) type HashData = Vec<u8>;
pub(crate) type PathData = std::path::PathBuf;
pub(crate) type FileSize = u64;

// macro_rules! vprintln {
//     ($($x:tt)*) => { println!($($x)*); }
// }
// macro_rules! vvprintln {
//     ($($x:tt)*) => { println!($($x)*); }
// }
macro_rules! vprintln {
    ($($x:tt)*) => {  }
}
macro_rules! vvprintln {
    ($($x:tt)*) => {  }
}
            
pub(crate) use {vprintln,vvprintln}; 
