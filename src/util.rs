#[allow(dead_code)]
pub fn is_set_msb(v: u16) -> bool {
    (v.rotate_left(1) & 1) == 1
}




