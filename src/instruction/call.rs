use hardware::emu::Emu;

pub fn call_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u16;
    let idx: u16 = if x == 0 {0} else {emu.gr[x] as u16};
    
    emu.sp.push(emu.pr);
    emu.pr = adr + idx;

}

#[cfg(test)]
mod tests {

    use emu::Emu;

    #[test]
    fn test_call_adr_x() {

        let mut emu = Emu::new();
        
        emu.gr[1] = 0xdead;
        emu.gr[2] = 0xbeef;
        emu.gr[3] = 0xcafe;

        emu.memory[0] = 0x8000;    // call memory[1000]
        emu.memory[1] = 1000;
        emu.memory[2] = 0x1413;    // LD GR1, GR3
        emu.memory[1000] = 0x1412; // 1000: LD GR1, GR2
        emu.memory[1001] = 0x8100; // ret
        
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);
        let code = emu.fetch();
        emu.execute(code);

        assert_eq!(emu.gr[1], 0xcafe);
        
    }
    
}
