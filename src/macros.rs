#[cfg(feature = "verbose")]
macro_rules! vprintln {
    ($($x:tt)*) => { println!($($x)*); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! vprintln {
    ($($x:tt)*) => {  }
}
pub(crate) use vprintln; 

#[cfg(feature = "very-verbose")]
macro_rules! vvprintln {
    ($($x:tt)*) => { println!($($x)*); }
}
#[cfg(not(feature = "very-verbose"))]
macro_rules! vvprintln {
    ($($x:tt)*) => {  }
}
pub(crate) use vvprintln; 
