use hardware::Emu;
use constant::*;

pub fn jpl_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if !emu.get_fr(SF) && !emu.get_fr(ZF) {
        emu.pr = adr + idx;
    }
    
}

pub fn jmi_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.get_fr(SF) {
        emu.pr = adr + idx;
    }
    
}

pub fn jnz_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if !emu.get_fr(ZF) {
        emu.pr = adr + idx;
    }
    
}

pub fn jze_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.get_fr(ZF) {
        emu.pr = adr + idx;
    }
    
}

pub fn jov_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.get_fr(OF) {
        emu.pr = adr + idx;
    }
    
}

pub fn jmp_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};
    
    emu.pr = adr + idx;
}

#[cfg(test)]
mod tests {
    
    use hardware::Emu;    
    use constant::*;

    #[test]
    fn test_jmi_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6100;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        emu.set_fr(SF, true);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

    #[test]
    fn test_jnz_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6200;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        emu.set_fr(ZF, false);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

    #[test]
    fn test_jze_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6300;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        emu.set_fr(ZF, true);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

    #[test]
    fn test_jmp_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6200;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

    #[test]
    fn test_jpl_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6200;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        emu.set_fr(SF, false);
        emu.set_fr(ZF, false);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

    #[test]
    fn test_jov_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6200;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        emu.set_fr(OF, true);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

}




