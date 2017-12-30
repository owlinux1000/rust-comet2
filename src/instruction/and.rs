use emu::Emu;
use util::*;

pub fn and_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;
    
    emu.gr[r1] &= emu.gr[r2];
    
    if emu.gr[r1] == 0 {
        emu.fr.zf = 1;
    } else {
        emu.fr.zf = 0;
    }
    
    if is_set_msb(emu.gr[r1]) {
        emu.fr.sf = 1;
    } else {
        emu.fr.sf = 0;
    }

    emu.fr.of = 0;
    
}


pub fn and_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};

    emu.gr[r] &= emu.memory[adr + idx];

    if emu.gr[r] == 0 {
        emu.fr.zf = 1;
    } else {
        emu.fr.zf = 0;
    }

    if is_set_msb(emu.gr[r]) {
        emu.fr.sf = 1;
    } else {
        emu.fr.sf = 0;
    }

    emu.fr.of = 0;
}

#[cfg(test)]
mod tests {

    use emu::{Emu,Fr};
    
    #[test]
    fn test_and_r1_r2() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x3412;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef & 0xdead);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});
    }
    
    #[test]
    fn test_and_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x3012;
        emu.memory[1] = 999;
        emu.memory[1000] = 0xbeef;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef & 0xdead);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});
    }

}
