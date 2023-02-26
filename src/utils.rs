pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) type HashData = Vec<u8>;
pub(crate) type PathData = std::path::PathBuf;
pub(crate) type FileSize = u64;

#[cfg(feature = "verbose")]
macro_rules! vprintln {
    ($($x:tt)*) => { println!($($x)*); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! vprintln {
    ($($x:tt)*) => {  }
}

#[cfg(feature = "very-verbose")]
macro_rules! vvprintln {
    ($($x:tt)*) => { println!($($x)*); }
}
#[cfg(not(feature = "very-verbose"))]
macro_rules! vvprintln {
    ($($x:tt)*) => {  }
}

pub(crate) use {vprintln,vvprintln}; 

