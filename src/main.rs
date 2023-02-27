use fdedup::{Deduplicator,Result,args::{Args,Parser}};

fn main() -> Result<()> {
    let args = Args::parse();
    Deduplicator::set_verbosity(args.verbose);
    let mut dedup = Deduplicator::default();
    for d in args.folders {
        dedup.add_dir(d);
    }
    dedup.normalize_path(args.normalize);
    if !args.disable_cache {
        dedup.read_cache(&args.cache_file);
    }
    dedup.run()?;
    if !args.disable_cache {
        dedup.write_cache(&args.cache_file)?;
    }
    Ok(())
}
