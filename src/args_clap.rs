use crate::{types::PathData, DEFAULT_CACHE_FILE};
use clap::Parser;

#[cfg(not(feature = "threads"))]
const HIDE_THREADS : bool = true;
#[cfg(feature = "threads")]
const HIDE_THREADS : bool = false;

#[cfg(not(feature = "verbose"))]
const HIDE_VERBOSE : bool = true;
#[cfg(feature = "verbose")]
const HIDE_VERBOSE : bool = false;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Folders to scan
    #[arg(default_value = ".")]
    pub folders: Vec<PathData>,

    /// Turn OFF caching of file hashes
    #[arg(short, long, default_value_t = false)]
    pub disable_cache: bool,
    
    /// Start with empty cache
    #[arg(short, long, default_value_t = false, conflicts_with="disable_cache")]
    pub empty_cache: bool,
    
    /// Where to store the cache
    #[arg(short, long, value_name = "<FILE>", default_value = DEFAULT_CACHE_FILE, conflicts_with="disable_cache")]
    pub cache_file: PathData,

    /// Normalize pathnames to Linux-style /
    #[arg(short, long, default_value_t = false)]
    pub normalize: bool,

    /// Number of computing threads to use (defaults to total cores)
    #[arg(short, long, hide=HIDE_THREADS)]
    pub threads: Option<usize>,

    /// Verbose output (repeat for more verbosity)
    #[arg(short='v', long="verbose", action = clap::ArgAction::Count, hide=HIDE_VERBOSE)]
    pub verbosity: u8,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}