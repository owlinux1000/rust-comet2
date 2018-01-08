use hardware::emu::Emu;

pub fn st_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    emu.memory[adr + idx] = emu.gr[r];
}

#[cfg(test)]
mod tests {
    use emu::Emu;

    #[test]
    fn test_st_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0x1;
        emu.memory[0] = 0x1112;
        emu.memory[1] = 999;
        emu.memory[1000] = 0xbeef;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.memory[1000], 0xdead);
    }
}
