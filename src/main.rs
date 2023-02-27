use fdedup::{Deduplicator,Result,args::{Args,Parser}};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut dedup = Deduplicator::default();
    #[cfg(feature = "verbose")]
    dedup.set_verbosity(args.verbosity);
    #[cfg(feature = "threads")]
    dedup.set_threads(args.threads);
    for d in args.folders {
        dedup.add_dir(d);
    }
    dedup.normalize_path(args.normalize);
    if !args.disable_cache && !args.empty_cache {
        dedup.read_cache(&args.cache_file);
    }
    dedup.run()?;
    if !args.disable_cache {
        dedup.write_cache(&args.cache_file)?;
    }
    Ok(())
}
