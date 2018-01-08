extern crate rust_comet2;
extern crate getopts;
extern crate libc;

use getopts::Options;

use rust_comet2::hardware::emu::Emu;
use rust_comet2::cli;
use rust_comet2::constant::{OF,SF,ZF};

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    
    cli::init_opts(&mut opts);
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || args.len() == 1 {
        println!("{}", opts.usage(&args[0]));
        std::process::exit(0);        
    }
    
    let mut codes = String::new();
    
    if !matches.free.is_empty() {
        cli::read_source_code(&mut codes, &matches.free[0]);
    }
    
    let mut emu = Emu::new();
    emu.debug_mode = matches.opt_present("d");
    let mut code_len = 0;
    
    for (i,line) in codes.lines().enumerate() {
        let hex = u16::from_str_radix(line, 16).unwrap();
        if i == 0 {
            code_len = hex
        } else {
            emu.memory[i-1] = hex
        }
    }

    for _ in 0..code_len {

        let code = emu.fetch();
        
        if emu.debug_mode {
            println!("Code\t0x{:0>4x}\t{:0>16b}", code, code);
        }
        
        emu.execute(code);
        
        if emu.debug_mode {
            for (i,gr) in emu.gr.iter_mut().enumerate() {
                println!("GR{}\t0x{:0>4x}\t{:0>16b}", i, gr, gr);
            }
            println!("OF\t{}", emu.get_fr(OF));
            println!("SF\t{}", emu.get_fr(SF));
            println!("ZF\t{}", emu.get_fr(ZF));
            println!("");
            unsafe {
                libc::getchar();
            }
        }

    }
    
}
