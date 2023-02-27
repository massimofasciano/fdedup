#[cfg(feature = "verbose")]
macro_rules! vprintln {
    ($verbosity:expr,$($x:tt)*) => { if $verbosity >= 1 { println!($($x)*); } }
}
#[cfg(feature = "verbose")]
macro_rules! vvprintln {
    ($verbosity:expr,$($x:tt)*) => { if $verbosity >= 2 { println!($($x)*); } }
//    ($($x:tt)*) => { if self.verbosity >= 2 { println!($($x)*); } }
}

#[cfg(not(feature = "verbose"))]
macro_rules! vprintln {
    ($verbosity:expr,$($x:tt)*) => {  }
}
#[cfg(not(feature = "verbose"))]
macro_rules! vvprintln {
    ($verbosity:expr,$($x:tt)*) => {  }
}

pub(crate) use vprintln; 
pub(crate) use vvprintln; 
