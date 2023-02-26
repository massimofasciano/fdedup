use fdedup::{Deduplicator, GenericResult};

fn main() -> GenericResult<()> {
    let cache = ".fdedup_cache.bin";
    let mut dedup = Deduplicator::default();
    dedup.add_dir(".");
    dedup.normalize_path(true);
    dedup.read_cache(cache);
    dedup.run()?;
    dedup.write_cache(cache)?;
    Ok(())
}
