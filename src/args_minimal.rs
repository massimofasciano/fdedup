use crate::{types::PathData, DEFAULT_CACHE_FILE};
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

impl Args {
    pub fn new() -> Self {
        let mut verbosity : u8 = 0;
        let mut empty_cache = false;
        let mut disable_cache = false;
        let mut normalize = false;
        let mut folders : Vec<PathData> = env::args().skip(1).filter_map(|arg| {
            match arg.as_str() {
                "-v" | "--verbose" => { verbosity += 1; None }
                "-vv" => { verbosity += 1; None }
                "-vvv" =>  { verbosity += 3; None }
                "-vvvv" =>  { verbosity += 4; None }
                "-vvvvv" =>  { verbosity += 5; None }
                "-e" | "--empty-cache" =>  { empty_cache = true; None }
                "-d" | "--disable-cache" =>  { disable_cache = true; None }
                "-n" | "--normalize" =>  { normalize = true; None }
                s => Some(PathData::from(s)),
            }
        }).collect();
        if folders.len() < 1 {
            folders = vec![PathData::from(".")];
        }
        Self {
            folders,
            disable_cache,
            empty_cache,
            cache_file : PathData::from(DEFAULT_CACHE_FILE),
            normalize,
            threads : Some(1),
            verbosity,
        }
    }
}

pub struct Parser;
