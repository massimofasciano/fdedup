pub mod types;
pub mod verbose;
pub mod duplicates;
pub mod hashedfile;
pub mod dedupstate;
pub mod deduplicator;
#[cfg(feature = "clap")]
pub mod args_clap;
#[cfg(not(feature = "clap"))]
pub mod args_minimal;

pub use types::Result;
pub use deduplicator::Deduplicator;
#[cfg(feature = "clap")]
pub use args_clap::Args;
#[cfg(not(feature = "clap"))]
pub use args_minimal::Args;
