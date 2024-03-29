pub mod types;
pub use types::Result;

pub mod verbose;
pub use verbose::set_verbosity;

pub mod duplicates;
pub use duplicates::Duplicates;
pub mod hashedfile;
pub mod dedupstate;

pub mod deduplicator;
pub use deduplicator::Deduplicator;

pub mod args;

pub const DEFAULT_CACHE_FILE : &str = ".fdedup_cache.bin";