use crate::{types::PathData, DEFAULT_CACHE_FILE};
use std::process::exit;
use std::env;

#[derive(Debug)]
pub struct Args {
    pub folders: Vec<PathData>,
    pub disable_cache: bool,
    pub empty_cache: bool,
    pub cache_file: PathData,
    pub normalize: bool,
    pub threads: Option<usize>,
    pub verbosity: u8,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} FOLDER... [options]", program);
    print!("{}", opts.usage(&brief));
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();
    
        let mut opts = getopts::Options::new();
        opts.optopt("c", "cache-file", format!("where to store the cache [default: {}]",DEFAULT_CACHE_FILE).as_str(), "FILE");
        #[cfg(feature = "threads")]
        opts.optopt("t", "threads", "mumber of computing threads to use (defaults to total cores)", "NUM");
        opts.optflag("h", "help", "print this help menu");
        opts.optflag("d", "disable-cache", "disable the cache");
        opts.optflag("e", "empty-cache", "start with an empty cache");
        opts.optflag("n", "normalize", "normalize pathnames to Linux-style /");
        #[cfg(feature = "verbose")]
        opts.optflagmulti("v", "verbose", "verbose output (repeat for more verbosity)");
        
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { 
                print_usage(&program, opts);
                println!("\n{}",f.to_string());
                exit(1)
            }
        };
        if matches.opt_present("h") {
            print_usage(&program, opts);
            exit(0);
        }
        let cache_file = PathData::from(matches.opt_str("c").unwrap_or(DEFAULT_CACHE_FILE.to_string()));
        #[cfg(not(feature = "threads"))]
        let threads = None;
        #[cfg(feature = "threads")]
        let threads = matches.opt_str("t").and_then(|s|s.parse::<usize>().ok());
        #[cfg(not(feature = "verbose"))]
        let verbosity = 0;
        #[cfg(feature = "verbose")]
        let verbosity = matches.opt_count("v") as u8;
        let empty_cache = matches.opt_present("e");
        let disable_cache = matches.opt_present("d");
        let normalize = matches.opt_present("n");
        let mut folders : Vec<PathData> = matches.free.iter().map(|s| PathData::from(s)).collect();
        if folders.len() < 1 {
            folders = vec![PathData::from(".")];
        }
        Self {
            folders,
            disable_cache,
            empty_cache,
            cache_file,
            normalize,
            threads,
            verbosity,
        }
    }
}

pub struct Parser;
