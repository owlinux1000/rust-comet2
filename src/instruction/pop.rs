use emu::Emu;

pub fn pop_r(emu: &mut Emu, code: u16) {
    let r = ((code & 0xf0) >> 4) as usize;
    emu.gr[r] = emu.sp.pop().unwrap();
}

#[cfg(test)]
mod tests {

    use emu::Emu;

    #[test]
    fn test_pop_r() {
        
        let mut emu = Emu::new();
        emu.sp.push(0xdead);
        emu.memory[0] = 0x7110;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0xdead);
        
    }
    
}
