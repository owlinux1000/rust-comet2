use hardware::emu::Emu;

pub fn ret(emu: &mut Emu) {
    emu.pr = emu.sp.pop().unwrap();
}
