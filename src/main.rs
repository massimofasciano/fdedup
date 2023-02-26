use fdedup::{Deduplicator,Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   folders: Vec<String>,
   #[arg(short, long, default_value_t = false)]
   cache: bool,
   #[arg(long, value_name = "<FILE>")]
   cache_file: Option<String>,
   #[arg(short, long, default_value_t = false)]
   normalize: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cache_default = ".fdedup_cache.bin";
    let mut dedup = Deduplicator::default();
    if args.folders.len() > 0 {
        for d in args.folders {
            dedup.add_dir(d.as_ref());
        }
    } else {
        dedup.add_dir(".");
    }
    dedup.normalize_path(args.normalize);
    if let Some(cache_file) = &args.cache_file {
        dedup.read_cache(cache_file);
    } else if args.cache {
        dedup.read_cache(cache_default);
    }
    dedup.run()?;
    if let Some(cache_file) = &args.cache_file {
        dedup.write_cache(cache_file)?;
    } else if args.cache {
        dedup.write_cache(cache_default)?;
    }
    Ok(())
}
