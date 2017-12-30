#[allow(dead_code)]
pub fn is_set_msb(v: u16) -> bool {
    let msb = (v & 0x8000) >> 15;
    msb == 1
}




