use emu::Emu;

pub fn jpl_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.fr.sf == 0 && emu.fr.zf == 0 {
        emu.pr = adr + idx;
    }
    
}

pub fn jmi_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.fr.sf == 1 {
        emu.pr = adr + idx;
        println!("{:0>16b}", emu.pr);
    }
    
}

pub fn jnz_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.fr.zf == 0 {
        emu.pr = adr + idx;
    }
    
}

pub fn jze_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.fr.zf == 1 {
        emu.pr = adr + idx;
    }
    
}

pub fn jov_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};

    if emu.fr.of == 1 {
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
    
    use emu::Emu;

    #[test]
    fn test_jmi_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.memory[0] = 0x6100;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0x1412;
        emu.fr.sf = 1; // if here is zero, test fails;
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
        emu.fr.zf = 0; // if here is zero, test fails;
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
        emu.fr.zf = 1; // if here is zero, test fails;
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
        emu.fr.sf = 0;
        emu.fr.zf = 0;
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
        emu.fr.of = 1;
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xbeef);
    }

}




