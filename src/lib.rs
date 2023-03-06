pub mod types;
pub use types::Result;

pub mod verbose;
pub use verbose::set_verbosity;

pub mod duplicates;
pub mod hashedfile;
pub mod dedupstate;

pub mod deduplicator;
pub use deduplicator::Deduplicator;

#[cfg(feature = "clap")]
pub mod args_clap;
#[cfg(feature = "clap")]
pub use args_clap::Args;

#[cfg(not(any(feature = "clap", feature = "getopts")))]
pub mod args_minimal;
#[cfg(not(any(feature = "clap", feature = "getopts")))]
pub use args_minimal::Args;

#[cfg(all(feature = "getopts",not(feature = "clap")))]
pub mod args_getopts;
#[cfg(all(feature = "getopts",not(feature = "clap")))]
pub use args_getopts::Args;

const DEFAULT_CACHE_FILE : &str = ".fdedup_cache.bin";