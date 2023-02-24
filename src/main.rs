use fdedup::{Deduplicator, GenericResult};

fn main() -> GenericResult<()> {
    let cache = format!(".fdedup_cache_{}.bin",std::path::MAIN_SEPARATOR as u8);
    let mut dedup = Deduplicator::new(".");
    dedup.read_cache(cache.as_str());
    dedup.run()?;
    dedup.write_cache(cache.as_str())?;
    Ok(())
}
