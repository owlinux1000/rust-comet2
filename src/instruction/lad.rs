use hardware::Emu;

pub fn lad_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x]};
    
    emu.gr[r] = adr + idx;
    
}

#[cfg(test)]
mod tests {
    
    use hardware::Emu;
    
    #[test]
    fn test_lad_r_adr_x() {
        
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x1212;
        emu.memory[1] = 999;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 1000);

    }
}
