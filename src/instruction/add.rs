use emu::Emu;
use util::*;

pub fn adda_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;
        
    match (emu.gr[r1] as i16).checked_add(emu.gr[r2] as i16) {
        
        Some(v) => {
            emu.gr[r1] = v as u16;
            emu.fr.of = 0;
        },
        None => {
            emu.gr[r1] = ((emu.gr[r1] as i16).wrapping_add(emu.gr[r2] as i16)) as u16;
            emu.fr.of = 1;
        }
    }
    
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
        
}

pub fn adda_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    let m = emu.memory[adr + idx];
    
    match (emu.gr[r] as i16).checked_add(m as i16) {
        
        Some(v) => {
            emu.gr[r] = v as u16;
            emu.fr.of = 0;
        }
        None => {
            emu.gr[r] = ((emu.gr[r] as i16).wrapping_add(m as i16)) as u16;
            emu.fr.of = 1;
        }
    }
    
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
        
}

pub fn addl_r1_r2(emu: &mut Emu, code: u16) {
    
    let r1 = ((code & 0xf0) >> 4) as usize;
    let r2 = (code & 0xf) as usize;
    
    match emu.gr[r1].checked_add(emu.gr[r2]) {
        Some(v) => {
            emu.gr[r1] = v;
            emu.fr.of = 0;
        },
        None => {
            emu.gr[r1] = emu.gr[r1].wrapping_add(emu.gr[r2]);
            emu.fr.of = 1;
        }
    }
    
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
}

pub fn addl_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    let m = emu.memory[adr + idx];
    
    match emu.gr[r].checked_add(m) {
        
        Some(v) => {
            emu.gr[r] = v;
            emu.fr.of = 0;
        }
        None => {
            emu.gr[r] = emu.gr[r].wrapping_add(m);
            emu.fr.of = 1;
        }
    }
    
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
}

#[cfg(test)]
mod tests {
    use emu::{Emu,Fr};

    #[test]
    fn test_adda_r1_r2() {

        // test for of, sf
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;
        emu.gr[2] = 1;
        emu.memory[0] = 0x2412;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x8000);
        assert_eq!(emu.fr, Fr{of: 1, sf: 1, zf:0});

        // test for zf
        let mut emu = Emu::new();
        emu.gr[1] = 0x0;
        emu.gr[2] = 0x0;
        emu.memory[0] = 0x2412;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x0);
        assert_eq!(emu.fr, Fr{of: 0, sf: 0, zf:1});
        
    }
    
    #[test]
    fn test_addl_r1_r2() {

        // test for of, sf
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;
        emu.gr[2] = 1;
        emu.memory[0] = 0x2612;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x8000);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});

        // test for zf
        let mut emu = Emu::new();
        emu.gr[1] = 0x0;
        emu.gr[2] = 0x0;
        emu.memory[0] = 0x2612;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x0);
        assert_eq!(emu.fr, Fr{of: 0, sf: 0, zf:1});
        
    }

    #[test]
    fn test_adda_r_adr_x() {
        
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x2012;
        emu.memory[1] = 999;
        emu.memory[1000] = 1;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x8000);
        assert_eq!(emu.fr, Fr{of: 1, sf: 1, zf:0});
    }

    #[test]
    fn test_addl_r_adr_x() {
        
        let mut emu = Emu::new();
        emu.gr[1] = 0x7fff;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x2212;
        emu.memory[1] = 999;
        emu.memory[1000] = 1;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0x8000);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});

    }

}
