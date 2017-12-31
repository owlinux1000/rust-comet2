use emu::Emu;
use util::*;
use constant::*;

pub fn ld_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;
    
    emu.gr[r1] = emu.gr[r2];
    
    let v = emu.gr[r1];
    emu.set_fr(ZF, v == 0);
    emu.set_fr(SF, is_set_msb(v));
    emu.set_fr(OF, false);
    
}

pub fn ld_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    emu.gr[r] = emu.memory[adr + idx];
    
    let v = emu.gr[r];
    emu.set_fr(ZF, v == 0);
    emu.set_fr(SF, is_set_msb(v));
    emu.set_fr(OF, false);
    
}

#[cfg(test)]
mod tests {
    
    use emu::Emu;
    
    #[test]
    fn test_ld_r1_r2() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x1412;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
        assert_eq!(emu.get_all_fr(), [false, true, false]);
    }
    
    #[test]
    fn test_ld_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x1012;
        emu.memory[1] = 999;
        emu.memory[1000] = 0xbeef;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
        assert_eq!(emu.get_all_fr(), [false, true, false]);
    }

}
