# fdedup
A recursive file contents deduplicator built in Rust:

- library crate with demo main program
- Uses SHA-512 to detect duplicates based on file contents
- Caches results to avoid rehashing (files are invalidated if modified date changes)

Sample binary will find all duplicates recursively (default is current folder and all subfolders).<br/>
Caching is enabled by default (default file is .fdedup_cache.bin)<br/>
Path normalization is enabled via -n (to the / Linux-style separator).

This is not production-level code. I wrote this project as a first learning experience with the Rust language and toolset.
In fact, a deduplicator is one of the projects I write to familiarize myself with a new programming language.

The code can be built for a multi-threaded environment where it takes advantage of parallel hashing of the files. Number of threads can be specified but defaults to the number of cores seen by the system. When file IO is the bottleneck, there is no huge advantage to threading unless dealing with a lot of very small files on a storage system that can handle many parallel requests. When most of the files are in the OS file cache, the speedup is huge.

The code can also be built without threads for systems that don't support them (some embedded systems, wasm-wasi at the moment of writing version 0.3 of this crate). There is no real speed advantage to building without threads if they are supported, even if you only have 1 core.

There are 3 versions of the dedupstate module based on mutex, dashmap and bare single threaded code. 

The single threaded code doesn't use interior mutability (compile time borrow checking). The single threaded version of dedupstate is also used when compiling a version of the code with threads and channels. A threadpool is used to create the file digests and send them to the main thread via channels. In this version the dedupstate is used in a single thread.

The mutex version places each of the 2 hashes inside a Mutex. It's almost a copy of the single threaded code but with immutable references to self and interior mutability. A special locked! macro is used to grab a lock on each mutex. This is the result of a failed attempt to compile the same code base with and without interior mutability. A normal function could have been used instead. The mutex version can also be compiled with the Mutex replaced by RefCell using a compile-time Cargo feature. Only the locked! macro changes. There is no noticeable speed difference in single threaded use.

The dashmap version replaces both HashMaps with DashMaps. Some code had to be rewritten slightly because of the differences between HashMap and DashMap. For this use case, the DashMap does not outperform the Mutex+HashMap. The bottlenecks are with the IO and the file digest hashing, which is not surprising.

So in summary, there are 4 setups:
- single threaded for all
- single threaded state + threaded file digests via channels (slightly slower than fully threaded but safer)
- multi-threaded state and digests using Mutex+HashMap
- multi-threaded state and digests using DashMap

The code can read and write a cache to remember the hashed file digests from previous runs. When possible, the last modified time is kept along with the digest and used to invalidate it if the file has changed since last being hashed. The caching leads to huge speed increases by skipping the 2 slow parts of the process: file IO and digest calculation. In the example program, it's also possible to skip the cache completely or to start with an empty cache and write a new version (skipping the existing one). The cache uses relative path names as specified to the program so the working directory is important. By default, it is stored in a file in the current directory. An option is provided to convert path separators to Linux-style "/" so that the cache can be shared between Windows and Linux (as long as relative paths are used relative to working dir).

The example program makes use of command line arguments via 3 optional libraries:
- a basic naive version (don't use this)
- a getopts version
- a clap version
The clap version is compiled by default and should be used but it didn't compile under wasm-wasi when 0.3 was written so the getopts version was added for this case.

This crate is fully functional when compiled for the wasm-wasi environment (WebAssembly System Interface). At the time when version 0.3 was written, clap and threads did not work so getopts and a single threaded algorithm had to be used. Wasi was tested using cargo wasi and wasmtime. It's important to use the "--dir" option to allow sandbox access to the folders that we are indexing and also the current directory for the cache file (if needed). Speed varies depending on the data and if it is in the OS cache or not. When everything is OS cached and we are not IO limited, it is about 50% slower than native single threaded code for calculating file digests. When we are IO limited, I have seen it run 5-6x slower than native.

Extra info about wasm-wasi:
- with rust nightly, clap compiles and works for wasi (tested on 2023-02-28)
- the features that use rayon (mutex and dashmap) work under wasi but I don't think they actually spawn any threads (it still works for my code because the main thread doesn't mind waiting)
- the channel+threadpool config crashes at runtime because wasi doesn't yet support real threads (it's coming)

A few compile-time features are available:
- default : native version by default
- native : by default selects ["clap", "mutex", "verbose"]. If mutex, channel or dashmap not selected, fully single threaded code is built.
- wasi : selects a set of features compatible with wasi
- verbose : optional verbosity controls (info messages)
- clap : best for arg parsing
- getopts : for arg parsing when clap doesn't compile
- channel : single threaded variant + threadpool just for the digest calculations
- mutex : Mutex+HashMap variant + rayon scope fully threaded
- dashmap : DashMap variant + rayon scope fully threaded
- refcell : use with channel to build the mutex variant with Mutex replaced by RefCell (RefCell+HashMap)

At this moment, no code was written to act on the duplicates, except to display them, so it's not a full deduplicator yet but rather a duplication detector. The next part is rather trivial and there are a few possibilities:
- erase all but one of the files in each group (or move them to a trash location)
- instead of erasing, hard-link to the first in the group
- instead of erasing, soft-link to the first in the group

This could be added to the duplicates module.

The demo program can be used to find duplicate files in a series of folders (recursively). It prints them in groups with their size and hex digest (SHA-512).

```
$ fdedup --help
Find groups of duplicate files by content

Usage: fdedup [OPTIONS] [FOLDERS]...

Arguments:
  [FOLDERS]...  Folders to scan [default: .]

Options:
  -d, --disable-cache        Turn OFF caching of file hashes
  -e, --empty-cache          Start with empty cache
  -c, --cache-file <<FILE>>  Where to store the cache [default: .fdedup_cache.bin]
  -n, --normalize            Normalize pathnames to Linux-style /
  -t, --threads <THREADS>    Number of computing threads to use  (defaults to total cores)
  -v, --verbose...           Verbose output (repeat for more verbosity)
  -h, --help                 Print help
  -V, --version              Print version

$ fdedup -V
fdedup 0.3.0

$ fdedup -n 

# 2199552 0682013c8c57565cd358fbe482f944ab7efc8b0ea0fd6740266a1fd5f2938f3433e7cdc74529bea7e2a35ad653befa1beedabc7f249f6cb620371e685fa05116
./target/debug/build/winapi-61fd0ec083e2af74/build_script_build.pdb
./target/debug/build/winapi-61fd0ec083e2af74/build_script_build-61fd0ec083e2af74.pdb

# 2355200 c3c5d21c4628dbd5d365eb8ace1442b5a719e697719ea791c647be796ccde56278ff594a4e00e0c17492c1d71b05d0a4d85783e18d68cb31d5b5da0af368d9b7
./target/debug/build/generic-array-1c07903af6f17199/build_script_build-1c07903af6f17199.pdb
./target/debug/build/generic-array-1c07903af6f17199/build_script_build.pdb
```

The demo program uses this fdedup crate:

```rust
use fdedup::{Deduplicator,Result,args::Args};
#[cfg(feature = "verbose")]
use fdedup::{set_verbosity};

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
```
