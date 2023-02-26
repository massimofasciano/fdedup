pub mod types;
pub mod macros;
pub mod duplicates;
pub mod hashedfile;
pub mod dedupstate;
pub mod deduplicator;
pub use types::Result;
pub use deduplicator::Deduplicator;
