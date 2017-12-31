extern crate rust_casl2;

use rust_casl2::emu::Emu;

fn main() {
    
    let mut emu = Emu::new();
    emu.gr[1] = 0x0;
    emu.gr[2] = 0xdead;
    emu.memory[0] = 0x1412; // LD GR1, GR2
    let code = emu.fetch();
    emu.execute(code);
    println!("{:x}", emu.gr[1]);
    println!("{:x}", emu.gr[2]);
    

    /*
    loop {
        
        let code = emu.fetch();

        if code == 0 {
            break;
        }
        
        emu.execute(code);
        
        println!("{:?}", emu.fr);
        println!("{:?}", emu.gr);
        break;
    }
     */
}
