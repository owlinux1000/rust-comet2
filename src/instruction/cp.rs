
use emu::Emu;

pub fn cpa_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;

    if (emu.gr[r1] as i16) > (emu.gr[r2] as i16) {
        emu.fr.sf = 0;
        emu.fr.zf = 0;
    }

    if (emu.gr[r1] as i16) == (emu.gr[r2] as i16) {
        emu.fr.sf = 0;
        emu.fr.zf = 1;
    }

    if (emu.gr[r1] as i16) < (emu.gr[r2] as i16) {
        emu.fr.sf = 1;
        emu.fr.zf = 0;
    }
    
    emu.fr.of = 0;
}

pub fn cpa_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    if (emu.gr[r] as i16) > (emu.memory[adr + idx] as i16) {
        emu.fr.sf = 0;
        emu.fr.zf = 0;
    }

    if (emu.gr[r] as i16) == (emu.memory[adr + idx] as i16) {
        emu.fr.sf = 0;
        emu.fr.zf = 1;
    }

    if (emu.gr[r] as i16) < (emu.memory[adr + idx] as i16) {
        emu.fr.sf = 1;
        emu.fr.zf = 0;
    }
    
    emu.fr.of = 0;
}

pub fn cpl_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;

    if emu.gr[r1] > emu.gr[r2] {
        emu.fr.sf = 0;
        emu.fr.zf = 0;
    }

    if emu.gr[r1] == emu.gr[r2] {
        emu.fr.sf = 0;
        emu.fr.zf = 1;
    }

    if emu.gr[r1] < emu.gr[r2] {
        emu.fr.sf = 1;
        emu.fr.zf = 0;
    }
    
    emu.fr.of = 0;
}

pub fn cpl_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    if emu.gr[r] > emu.memory[adr + idx] {
        emu.fr.sf = 0;
        emu.fr.zf = 0;
    }

    if emu.gr[r] == emu.memory[adr + idx] {
        emu.fr.sf = 0;
        emu.fr.zf = 1;
    }

    if emu.gr[r] < emu.memory[adr + idx] {
        emu.fr.sf = 1;
        emu.fr.zf = 0;
    }
    
    emu.fr.of = 0;
}

#[cfg(test)]
mod tests {

    use emu::{Emu,Fr};
    
    #[test]
    fn test_cpa_r1_r2() {
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;        
        emu.gr[2] = 0x8000;
        emu.memory[0] = 0x4312;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.fr, Fr{of: 0, sf: 0, zf:0});
    }
    
    #[test]
    fn test_cpa_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x4012;
        emu.memory[1] = 999;
        emu.memory[1000] = 0x8000;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.fr, Fr{of: 0, sf: 0, zf:0});
    }

    #[test]
    fn test_cpl_r1_r2() {
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;        
        emu.gr[2] = 0x8000;
        emu.memory[0] = 0x4512;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});
    }
    
    #[test]
    fn test_cpl_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x4112;
        emu.memory[1] = 999;
        emu.memory[1000] = 0x8000;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});
    }

}

    

