pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// Find groups of duplicate files by content
pub struct Args {
    /// Folders to scan
    pub folders: Vec<String>,

    /// Turn OFF caching of file hashes
    #[arg(short, long, default_value_t = false)]
    pub disable_cache: bool,
    
    /// Where to store the cache
    #[arg(long, value_name = "<FILE>", default_value_t = String::from(".fdedup_cache.bin"))]
    pub cache_file: String,

    /// Normalize pathnames to Linux-style /
    #[arg(short, long, default_value_t = false)]
    pub normalize: bool,

    /// Verbose output (repeat for more verbosity)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
