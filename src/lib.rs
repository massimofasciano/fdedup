pub mod types;
pub mod verbose;
pub mod duplicates;
pub mod hashedfile;
pub mod dedupstate;
pub mod deduplicator;
#[cfg(feature = "clap")]
pub mod args_clap;
#[cfg(feature = "minimal_args")]
pub mod args_minimal;
#[cfg(feature = "getopts")]
pub mod args_getopts;

pub use types::Result;
pub use deduplicator::Deduplicator;
#[cfg(feature = "clap")]
pub use args_clap::Args;
#[cfg(feature = "minimal_args")]
pub use args_minimal::Args;
#[cfg(feature = "getopts")]
pub use args_getopts::Args;
pub use verbose::set_verbosity;

const DEFAULT_CACHE_FILE : &str = ".fdedup_cache.bin";