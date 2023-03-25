use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::{Result};

pub(crate) static VERBOSITY: Lazy<Arc<Mutex<u8>>> = Lazy::new(|| {
    Arc::new(Mutex::new(0))
});

#[cfg(feature = "verbose")]
pub (crate) fn check_verbosity(verbosity : u8) -> bool {
    if let Ok(global_verbosity) = VERBOSITY.lock() {
        verbosity <= *global_verbosity
    } else {
        false
    }
}

pub fn set_verbosity(verbosity : u8) -> Result<()> {
    match VERBOSITY.lock() {
        Ok(mut global_verbosity) => {
            *global_verbosity = verbosity;
            Ok(())
        },
        Err(e) => Err(anyhow::format_err!("{}",e)),
    }
}

#[cfg(feature = "verbose")]
macro_rules! vprintln {
    ($verbosity:expr,$($x:tt)*) => { if crate::verbose::check_verbosity($verbosity) { println!($($x)*); } }
}

#[cfg(not(feature = "verbose"))]
macro_rules! vprintln {
    ($verbosity:expr,$($x:tt)*) => {  }
}

pub(crate) use vprintln; 

