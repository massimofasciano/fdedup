# fdedup
Deduplicator (Rust lib+bin)

- Uses SHA-512 to detect duplicates based on file contents
- Caches results to avoid rehashing (files are invalidated if modified date changes)

Sample binary will find all duplicates recursively (default is current folder and all subfolders).<br/>
Caching is enabled by default (default file is .fdedup_cache.bin)<br/>
Path normalization is enabled via -n (to the / Linux-style separator).

```
$ fdedup --help

Find groups of duplicate files by content

Usage: fdedup [OPTIONS] [FOLDERS]...

Arguments:
  [FOLDERS]...  Folders to scan [default: .]

Options:
  -d, --disable-cache        Turn OFF caching of file hashes
  -c, --cache-file <<FILE>>  Where to store the cache [default: .fdedup_cache.bin]
  -n, --normalize            Normalize pathnames to Linux-style /
  -v, --verbose...           Verbose output (repeat for more verbosity)
  -h, --help                 Print help
  -V, --version              Print version

$ fdedup -n 

# 2199552 0682013c8c57565cd358fbe482f944ab7efc8b0ea0fd6740266a1fd5f2938f3433e7cdc74529bea7e2a35ad653befa1beedabc7f249f6cb620371e685fa05116
./target/debug/build/winapi-61fd0ec083e2af74/build_script_build.pdb
./target/debug/build/winapi-61fd0ec083e2af74/build_script_build-61fd0ec083e2af74.pdb

# 2355200 c3c5d21c4628dbd5d365eb8ace1442b5a719e697719ea791c647be796ccde56278ff594a4e00e0c17492c1d71b05d0a4d85783e18d68cb31d5b5da0af368d9b7
./target/debug/build/generic-array-1c07903af6f17199/build_script_build-1c07903af6f17199.pdb
./target/debug/build/generic-array-1c07903af6f17199/build_script_build.pdb
```

Can also be used as a library:

```rust
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
```
