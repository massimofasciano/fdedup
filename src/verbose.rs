#[cfg(feature = "verbose")]
macro_rules! vprintln {
    ($target:expr,$verbosity:expr,$($x:tt)*) => { if $verbosity >= $target { println!($($x)*); } }
}

#[cfg(not(feature = "verbose"))]
macro_rules! vprintln {
    ($target:expr,$verbosity:expr,$($x:tt)*) => {  }
}

pub(crate) use vprintln; 
