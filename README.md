# fdedup
Deduplicator (Rust lib+bin)

- Uses SHA-512 to detect duplicates based on file contents
- Can cache results to avoid rehashing (files are invalidated if modified date changes)

Sample binary will find all duplicates in current folder and all subfolders.
Caching is enabled using .fdedup_cache_NN.bin (NN=47 on Linux and 92 on Windows based on path separator character code).

```bash
# fdedup

# 2199552 0682013c8c57565cd358fbe482f944ab7efc8b0ea0fd6740266a1fd5f2938f3433e7cdc74529bea7e2a35ad653befa1beedabc7f249f6cb620371e685fa05116
./target/debug/build/winapi-61fd0ec083e2af74/build_script_build.pdb
./target/debug/build/winapi-61fd0ec083e2af74/build_script_build-61fd0ec083e2af74.pdb

# 2355200 c3c5d21c4628dbd5d365eb8ace1442b5a719e697719ea791c647be796ccde56278ff594a4e00e0c17492c1d71b05d0a4d85783e18d68cb31d5b5da0af368d9b7
./target/debug/build/generic-array-1c07903af6f17199/build_script_build-1c07903af6f17199.pdb
./target/debug/build/generic-array-1c07903af6f17199/build_script_build.pdb

```

Can also be used as a library:

```rust
use fdedup::{Deduplicator, GenericResult};

fn main() -> GenericResult<()> {
    let cache = format!(".fdedup_cache_{}.bin",std::path::MAIN_SEPARATOR as u8);
    let mut dedup = Deduplicator::new(".");
    dedup.read_cache(cache.as_str());
    dedup.run()?;
    dedup.write_cache(cache.as_str())?;
    Ok(())
}
```
