pub mod types;
pub use types::Result;

pub mod verbose;
pub use verbose::set_verbosity;

pub mod duplicates;
pub mod hashedfile;
pub mod dedupstate;

pub mod deduplicator;
pub use deduplicator::Deduplicator;

pub mod args;

const DEFAULT_CACHE_FILE : &str = ".fdedup_cache.bin";