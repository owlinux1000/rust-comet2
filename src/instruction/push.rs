use hardware::emu::Emu;

pub fn push_adr_x(emu: &mut Emu, code: u16) {
    
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as usize;
    let idx: usize = if x == 0 {0} else {emu.gr[x] as usize};
    
    emu.sp.push(emu.memory[adr + idx]);
    
}

#[cfg(test)]
mod tests {

    use emu::Emu;

    #[test]
    fn test_push_adr_x() {
        let mut emu = Emu::new();
        emu.memory[0] = 0x7000;
        emu.memory[1] = 1000;
        emu.memory[1000] = 0xdead;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.sp.pop().unwrap(), 0xdead);
        
    }
    
}
