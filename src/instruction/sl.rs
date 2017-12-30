use emu::Emu;
use util::*;

pub fn sla_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u32;
    let idx: u32 = if x == 0 {0} else {emu.gr[x] as u32};
    
    let shift_bit = adr + idx;

    // 1bit余分にbit回転して末尾1byteを取ることで14bit目を取得
    emu.fr.of = (emu.gr[r].rotate_left(shift_bit + 1) & 0x1) as u8;

    if is_set_msb(emu.gr[r]) {
        // 元のMSBが1だったらMSBを1でorして強制的に1にする
        emu.gr[r] = (emu.gr[r] << shift_bit) | 0x8000;
    } else {
        // 元のMSBが0だったらMSBを0でandして強制的に0にする
        emu.gr[r] = (emu.gr[r] << shift_bit) & 0x7fff;
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

pub fn sll_r_adr_x(emu: &mut Emu, code: u16) {

    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u32;
    let idx: u32 = if x == 0 {0} else {emu.gr[x] as u32};

    let shift_bit = adr + idx;
    
    // 1bit余分にbit回転して末尾1byteを取ることで14bit目を取得
    emu.fr.of = (emu.gr[r].rotate_left(shift_bit) & 0x1) as u8;
    emu.gr[r] <<= shift_bit;
    
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

pub fn sra_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u32;
    let idx: u32 = if x == 0 {0} else {emu.gr[x] as u32};

    let shift_bit = adr + idx;

    emu.fr.of = ((emu.gr[r].rotate_right(shift_bit) & 0x8000) >> 15) as u8;
    
    emu.gr[r] = ((emu.gr[r] as i16) >> shift_bit) as u16;


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

    emu.fr.of = 0;
}

pub fn srl_r_adr_x(emu: &mut Emu, code: u16) {
    
    let r = ((code & 0xf0) >> 4) as usize;
    let x = (code & 0xf) as usize;
    let adr = emu.fetch() as u32;
    let idx: u32 = if x == 0 {0} else {emu.gr[x] as u32};

    let shift_bit = adr + idx;

    println!("{:0>16b}", emu.gr[r].rotate_right(shift_bit));
    emu.fr.of = ((emu.gr[r].rotate_right(shift_bit) & 0x8000) >> 15) as u8;
    emu.gr[r] >>= shift_bit;

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

    use emu::{Emu, Fr};
    
    #[test]
    fn test_sla_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0b1111111101001000;
        emu.gr[2] = 3;
        emu.memory[0] = 0x5012;
        emu.memory[1] = 0x0;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0b1111101001000000);
        assert_eq!(emu.fr, Fr{of: 1, sf: 1, zf:0});
    }
    
    #[test]
    fn test_sra_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 0b1111111101001000;
        emu.gr[2] = 3;
        emu.memory[0] = 0x5112;
        emu.memory[1] = 0x0;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 0b1111111111101001);
        assert_eq!(emu.fr, Fr{of: 0, sf: 1, zf:0});
    }
    
    #[test]
    fn test_sll_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 65352;
        emu.gr[2] = 3;
        emu.memory[0] = 0x5212;
        emu.memory[1] = 0x0;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 64064);
        assert_eq!(emu.fr, Fr{of: 1, sf: 1, zf:0});
    }
    
    #[test]
    fn test_srl_r_adr_x() {
        let mut emu = Emu::new();
        emu.gr[1] = 65352;        
        emu.gr[2] = 3;
        emu.memory[0] = 0x5312;
        emu.memory[1] = 0x0;
        let code = emu.fetch();
        emu.execute(code);
        assert_eq!(emu.gr[1], 8169);        
        assert_eq!(emu.fr, Fr{of: 0, sf: 0, zf:0});
    }
    
}
