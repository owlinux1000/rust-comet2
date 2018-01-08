extern crate rust_comet2;

use rust_comet2::emu::Emu;
use rust_comet2::cli;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let mut codes = String::new();
    
    match cli::arg_parse(&args) {
        
        cli::Func::Help => {
            
            println!("Usage: rust-comet2 <filename>");
            std::process::exit(0);
            
        },
        
        cli::Func::Execute => {
            
            use std::fs::File;            
            use std::path::Path;
            use std::io::prelude::*;
            
            let path = Path::new(&args[1]);
            let mut file = File::open(&path).unwrap();
            file.read_to_string(&mut codes).unwrap();
            
        },
        
        cli::Func::Error => {
            
            std::process::exit(1);
            
        },
    };
    
    let mut emu = Emu::new();
    let mut code_len = 0;
    
    for (i,line) in codes.lines().enumerate() {
        let hex = u16::from_str_radix(line, 16).unwrap();
        if i == 0 {
            code_len = hex
        } else {
            emu.memory[i-1] = hex
        }
    }

    let mut i = 0;
    
    while i < code_len {

        let code = emu.fetch();
        println!("Memory {:0>4x}", code);
        if code == 0 {
            break;
        }
        
        emu.execute(code);
        i += 1;

        println!("GR {:?}", emu.gr);
    }
    
}
