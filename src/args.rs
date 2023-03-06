#[cfg(feature = "clap")]
pub mod clap;
#[cfg(feature = "clap")]
pub use self::clap::Args;

#[cfg(not(any(feature = "clap", feature = "getopts")))]
pub mod basic;
#[cfg(not(any(feature = "clap", feature = "getopts")))]
pub use self::basic::Args;

#[cfg(all(feature = "getopts",not(feature = "clap")))]
pub mod getopts;
#[cfg(all(feature = "getopts",not(feature = "clap")))]
pub use self::getopts::Args;
