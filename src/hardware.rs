use instruction::*;

pub const ZF: u8 = 0b001;
pub const SF: u8 = 0b010;
pub const OF: u8 = 0b100;

#[derive(Debug)]
struct Fr {
    of: bool,
    sf: bool,
    zf: bool,
}

impl Fr {
    
    pub fn new() -> Fr {
        Fr{of: false, sf: false, zf: false}
    }
    
    pub fn set(&mut self, i: u8, f: bool) {

        match i {
            ZF => self.zf = f,
            SF => self.sf = f,
            OF => self.of = f,
            _ => panic!("Invalid flag register"),
        };
        
    }

    pub fn get_all(&self) -> [bool; 3] {
        [self.of, self.sf, self.zf]
    }
    
    pub fn get(&self, i: u8) -> bool {
        
        match i {
            ZF => self.zf,
            SF => self.sf,
            OF => self.of,
            _ => panic!("Invalid flag register"),
        }
        
    }
    
}

pub struct Emu{
    
    pub memory: [u16; 65536],
    
    // General Register
    pub gr: [u16; 8],
    
    // Program Register
    pub pr: u16,
    
    // Stack Pointer
    pub sp: Vec<u16>,
    
    // Flag Register [OF, SF, ZF]
    fr: [bool; 3],

    pub debug_mode: bool,
}

impl Emu {
    
    pub fn new() -> Emu {
        Emu{
            memory: [0; 65536],
            gr: [0; 8],
            pr: 0,
            sp: Vec::new(),
            fr: [false; 3],
            debug_mode: false,
        }
    }
    
    pub fn fetch(&mut self) -> u16 {
        
        let code = self.memory[self.pr as usize];
        self.pr += 1;
        
        code
    }
    
    pub fn execute(&mut self, code: u16) {
        
        let op = (code & 0xff00) >> 8;
        
        if self.debug_mode {
            println!("Opcode\t0x{:0>2x}\t{:0>8b}", op, op);
        }
        
        match op {
            
            0x10 => {
                ld::ld_r_adr_x(self, code);
            },
            0x11 => {
                st::st_r_adr_x(self, code);
            },
            0x12 => {
                lad::lad_r_adr_x(self, code);
            },
            0x14 => {
                ld::ld_r1_r2(self, code);
            },
            0x20 => {
                add::adda_r_adr_x(self, code);
            },
            0x21 => {
                sub::suba_r_adr_x(self, code);
            },
            0x22 => {
                add::addl_r_adr_x(self, code);
            },
            0x23 => {
                sub::subl_r_adr_x(self, code);
            },
            0x24 => {
                add::adda_r1_r2(self, code);
            },
            0x25 => {
                sub::suba_r1_r2(self, code);
            },
            0x26 => {
                add::addl_r1_r2(self, code);
            },
            0x27 => {
                sub::subl_r1_r2(self, code);
            },
            0x30 => {
                and::and_r_adr_x(self, code);
            },
            0x31 => {
                or::or_r_adr_x(self, code);
            },
            0x32 => {
                xor::xor_r_adr_x(self, code);
            },
            0x34 => {
                and::and_r1_r2(self, code);
            },
            0x35 => {
                or::or_r1_r2(self, code);
            },
            0x36 => {
                xor::xor_r1_r2(self, code);
            },
            0x40 => {
                cp::cpa_r_adr_x(self, code);
            },
            0x41 => {
                cp::cpl_r_adr_x(self, code);
            },
            0x43 => {
                cp::cpa_r1_r2(self, code);
            },
            0x45 => {
                cp::cpl_r1_r2(self, code);
            },
            0x50 => {
                sl::sla_r_adr_x(self, code);
            },
            0x51 => {
                sl::sra_r_adr_x(self, code);
            },
            0x52 => {
                sl::sll_r_adr_x(self, code);
            },
            0x53 => {
                sl::srl_r_adr_x(self, code);
            },
            0x61 => {
                jmp::jmi_adr_x(self, code);
            },
            0x62 => {
                jmp::jnz_adr_x(self, code);
            },
            0x63 => {
                jmp::jze_adr_x(self, code);
            },
            0x64 => {
                jmp::jmp_adr_x(self, code);
            },
            0x65 => {
                jmp::jpl_adr_x(self, code);
            },
            0x66 => {
                jmp::jov_adr_x(self, code);
            },
            0x70 => {
                push::push_adr_x(self, code);
            },
            0x71 => {
                pop::pop_r(self, code);
            },
            0x80 => {
                call::call_adr_x(self, code);
            },
            0x81 => {
                ret::ret(self);
            },
            0xf0 => {
                svc::svc_adr_x(self, code);                
            },
            _ => {
                println!("Not implemented.");
            }
        }
    }
    
    pub fn set_fr(&mut self, i: usize, flag: bool) {
        
        if i > 2 {
            println!("Invalid flag register.")
        }
        self.fr[i] = flag;
        
    }

    pub fn get_fr(&self, i: usize) -> bool {
        
        if i > 2 {
            println!("Invalid flag register.")
        }
        self.fr[i]
            
    }

    pub fn get_all_fr(&self) -> [bool; 3] {
        self.fr
    }
    
}

