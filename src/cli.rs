extern crate getopts;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use self::getopts::Options;

pub fn init_opts(opts: &mut Options) {
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("d", "debug", "execute debugging mode");
}

pub fn read_source_code(buf: &mut String, path: &str) {
    let path = Path::new(path);
    let mut file = File::open(path).unwrap();
    file.read_to_string(buf).unwrap();
}
        
