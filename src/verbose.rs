pub (crate) static mut VERBOSITY : u8 = 0;

#[cfg(feature = "verbose")]
macro_rules! vprintln {
    ($($x:tt)*) => { if unsafe { crate::verbose::VERBOSITY >= 1 } { println!($($x)*); } }
}
#[cfg(not(feature = "verbose"))]
macro_rules! vprintln {
    ($($x:tt)*) => {  }
}
pub(crate) use vprintln; 

#[cfg(feature = "very-verbose")]
macro_rules! vvprintln {
    ($($x:tt)*) => { if unsafe { crate::verbose::VERBOSITY >= 2 } { println!($($x)*); } }
}
#[cfg(not(feature = "very-verbose"))]
macro_rules! vvprintln {
    ($($x:tt)*) => {  }
}
pub(crate) use vvprintln; 
