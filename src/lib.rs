pub mod types;
pub mod verbose;
pub mod duplicates;
pub mod hashedfile;
pub mod dedupstate;
pub mod deduplicator;
pub mod args;
pub use types::Result;
pub use deduplicator::Deduplicator;
