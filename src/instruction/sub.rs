use hardware::emu::Emu;
use util::*;
use constant::*;

pub fn suba_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;
    
    match (emu.gr[r1] as i16).checked_sub(emu.gr[r2] as i16) {
        
        Some(v) => {
            emu.gr[r1] = v as u16;
            emu.set_fr(OF, false);
        }
        None => {
            emu.gr[r1] = ((emu.gr[r1] as i16).wrapping_sub(emu.gr[r2] as i16)) as u16;
            emu.set_fr(OF, true);            
        }
    }

    let v = emu.gr[r1];
    emu.set_fr(ZF, v == 0);
    emu.set_fr(SF, is_set_msb(v));
}

pub fn suba_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};

    let m = emu.memory[adr + idx];
    
    match (emu.gr[r] as i16).checked_sub(m as i16) {
        
        Some(v) => {
            emu.gr[r] = v as u16;
            emu.set_fr(OF, false);
        }
        None => {
            emu.gr[r] = ((emu.gr[r] as i16).wrapping_sub(m as i16)) as u16;
            emu.set_fr(OF, true);            
        }
    }

    let v = emu.gr[r];
    emu.set_fr(ZF, v == 0);
    emu.set_fr(SF, is_set_msb(v));
        
}

pub fn subl_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;
    
    match emu.gr[r1].checked_sub(emu.gr[r2]) {
        Some(v) => {
            emu.gr[r1] = v;
            emu.set_fr(OF, false);
        },
        None => {
            emu.gr[r1] = emu.gr[r1].wrapping_sub(emu.gr[r2]);
            emu.set_fr(OF, true);
        }
    }
    
    let v = emu.gr[r1];
    emu.set_fr(ZF, v == 0);
    emu.set_fr(SF, is_set_msb(v));
    
}

pub fn subl_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;    
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    let m = emu.memory[adr + idx];
    
    match emu.gr[r].checked_sub(m) {
        
        Some(v) => {
            emu.gr[r] = v;
            emu.set_fr(OF, false);
        }
        None => {
            emu.gr[r] = emu.gr[r].wrapping_sub(m);
            emu.set_fr(OF, true);
        }
    }
    
    let v = emu.gr[r];
    emu.set_fr(ZF, v == 0);
    emu.set_fr(SF, is_set_msb(v));
    
}

#[cfg(test)]
mod tests {
    
    use emu::Emu;

    #[test]
    fn test_suba_r1_r2() {

        // test for of, sf
        let mut emu = Emu::new();
        emu.gr[1] = 0x8000;
        emu.gr[2] = 1;
        emu.memory[0] = 0x2512;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x7fff);
        assert_eq!(emu.get_all_fr(), [true, false, false]);
        
    }
    
    #[test]
    fn test_subl_r1_r2() {

        // test for of, sf
        let mut emu = Emu::new();
        emu.gr[1] = 0x8000;
        emu.gr[2] = 1;
        emu.memory[0] = 0x2712;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x7fff);
        assert_eq!(emu.get_all_fr(), [false, false, false]);
        
    }

    #[test]
    fn test_suba_r_adr_x() {
        
        let mut emu = Emu::new();
        emu.gr[1] = 0x8000;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x2112;
        emu.memory[1] = 999;
        emu.memory[1000] = 1;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x7fff);
        assert_eq!(emu.get_all_fr(), [true, false, false]);
        
    }

    #[test]
    fn test_subl_r_adr_x() {
        
        let mut emu = Emu::new();
        emu.gr[1] = 0x8000;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x2312;
        emu.memory[1] = 999;
        emu.memory[1000] = 1;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x7fff);
        assert_eq!(emu.get_all_fr(), [false, false, false]);

    }

}
