pub use clap::Parser;
use crate::types::PathData;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Folders to scan
    #[arg(default_value = ".")]
    pub folders: Vec<PathData>,

    /// Turn OFF caching of file hashes
    #[arg(short, long, default_value_t = false)]
    pub disable_cache: bool,
    
    /// Where to store the cache
    #[arg(short, long, value_name = "<FILE>", default_value = ".fdedup_cache.bin")]
    pub cache_file: PathData,

    /// Normalize pathnames to Linux-style /
    #[arg(short, long, default_value_t = false)]
    pub normalize: bool,

    /// Verbose output (repeat for more verbosity)
    #[arg(short='v', long="verbose", action = clap::ArgAction::Count, hide=!cfg!(verbose))]
    pub verbosity: u8,
}
