extern crate rust_casl2;

mod util;
use rust_casl2::emu::Emu;

fn main() {
    
    let mut emu = Emu::new();
    
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
}
