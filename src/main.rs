use fdedup::{Deduplicator,Result,Args,set_verbosity};

fn main() -> Result<()> {
    let args = Args::new();
    #[cfg(feature = "verbose")]
    set_verbosity(args.verbosity)?;
    let mut dedup = Deduplicator::default();
    #[cfg(feature = "threads")]
    dedup.set_threads(args.threads);
    for d in args.folders {
        dedup.add_dir(d);
    }
    dedup.set_normalize_path(args.normalize);
    if !args.disable_cache && !args.empty_cache {
        dedup.read_cache(&args.cache_file);
    }
    let duplicates = dedup.run()?;
    if !args.disable_cache {
        dedup.write_cache(&args.cache_file)?;
    }
    for dup in duplicates {
        println!("{}",dup);
    }
    Ok(())
}
