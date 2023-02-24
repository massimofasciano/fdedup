use fdedup::{Deduplicator, GenericResult};

fn main() -> GenericResult<()> {
    let cache = "cache.bin";
    let mut dedup = Deduplicator::new(".");
    dedup.read_cache(cache);
    dedup.run()?;
    dedup.write_cache(cache)?;
    Ok(())
}
