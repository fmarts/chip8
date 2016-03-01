#![allow(dead_code)]

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fmt::{Debug, Formatter, Result as FmtResult};

use rand::{thread_rng, Rng};
use sdl2::Sdl;
use sdl2::rect::Rect;
use ep::FromPrimitive;

use screen::Screen;
use opcodes::Opcodes;


// TODO: move this elsewhere
const FONT_SET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0x80, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

// TODO: move this elsewhere
#[derive(Default)]
struct Instruction {
    opcode: u16,
    nnn:    u16,
    kk:     u8,
    x:      usize, 
    y:      usize,
    n:      u8,
}

impl Instruction {
    fn new() -> Instruction {
        Instruction::default()
    }

    fn decode(&mut self, data: (u8,u8)) {
        let fstb = data.0;
        let sndb = data.1;

        let op = fstb as u16 >> 4;

        match op {
            0x00                                => {
                self.opcode = op << 12 | sndb as u16;
            },
            0x01 | 0x02 | 0x0a | 0x0b           => {
                self.opcode = op << 12;
                self.nnn    = (fstb as u16 & 0x0F) << 8 | sndb as u16;
            },
            0x03 | 0x04 | 0x06 | 0x07 | 0x0c    => {
                self.opcode = op << 12;
                self.x      = (fstb & 0x0F) as usize;
                self.kk     = sndb;
            },
            0x05 | 0x08 | 0x09                  => {
                self.opcode = op << 12 | sndb as u16 & 0x0F;
                self.x      = (fstb & 0x0F) as usize;
                self.y      = ((sndb & 0xF0) >> 4) as usize;
            },
            0x0e | 0x0f                         => {
                self.opcode = op << 12 | sndb as u16;
                self.x      = (fstb & 0x0F) as usize;
            },
            0x0d                                => {
                self.opcode = op << 12;
                self.x      = (fstb & 0x0F) as usize;
                self.y      = ((sndb & 0xF0) >> 4) as usize;
                self.n      = sndb & 0x0F;
            },
            _                                   => {
                self.opcode = 0xFFFF;
            },
        }
    }

    fn get_opcode(&self) -> u16 {
        self.opcode
    }

    fn set_opcode(&mut self, val: u16) {
        self.opcode = val;
    }
}

pub struct Chip8<'a> {
    regs:   [u8; 16],
    i:      u16, 
    dt:     u8,
    st:     u8,
    pc:     usize,

    inst:   Instruction,
    jmp:    bool,

    mem:    [u8; 4096],
    stack:  Vec<u16>,
    screen: Screen<'a>,
}

impl<'a> Debug for Chip8<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
         self.regs[..].fmt(f)
    }
}

impl<'a> Chip8<'a> {
    pub fn new(sdl: &Sdl) -> Chip8<'a> {
        Chip8 {
            regs:   [0; 16],
            i:      0,
            dt:     0,
            st:     0,
            pc:     0x200,
            inst:   Instruction::new(),
            jmp:    false,
            mem:    [0; 4096], 
            stack:  vec![],
            screen: Screen::new(sdl),
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(Path::new(path)).unwrap();
        file.read_to_end(&mut rom).unwrap();
        for i in 0usize..rom.len() {
            self.mem[0x200 + i] = rom[i]; 
        }
        // TODO: move this
        for i in 0usize..FONT_SET.len() {
            self.mem[0x00 + i] = FONT_SET[i];
        }
    }

    pub fn fetch(&mut self) {
        let raw_data = (self.mem[self.pc], self.mem[self.pc +1]); 
        self.inst.decode(raw_data); 

        self.jmp = false;

        println!("{:#x}: {:#x}", self.pc, self.inst.get_opcode());
        
        match Opcodes::from_u16(self.inst.get_opcode()).unwrap() {
            Opcodes::CLS    => self.cls(),
            Opcodes::RET    => self.ret(),
            Opcodes::JMP    => self.jmp(),
            Opcodes::JMP_VA => self.jmp_va(),
            Opcodes::CALL   => self.call(),
            Opcodes::SE_VB  => self.se_vb(),
            Opcodes::SE_VV  => self.se_vv(),
            Opcodes::SNE_VB => self.sne_vb(),
            Opcodes::SNE_VV => self.sne_vv(),
            Opcodes::OR     => self.or(),
            Opcodes::ADD_VB => self.add_vb(),
            Opcodes::ADD_VV => self.add_vv(),
            Opcodes::ADD_IV => self.add_iv(),
            Opcodes::SUB    => self.sub(),
            Opcodes::XOR    => self.xor(),
            Opcodes::AND    => self.and(),
            Opcodes::LD_VB  => self.ld_vb(),
            Opcodes::LD_BV  => self.ld_bv(),
            Opcodes::LD_VV  => self.ld_vv(),
            Opcodes::LD_VI  => self.ld_vi(),
            Opcodes::LD_IV  => self.ld_iv(),
            Opcodes::LD_FV  => self.ld_fv(),
            Opcodes::LD_IA  => self.ld_ia(),
            Opcodes::LD_VDT => self.ld_vdt(),
            Opcodes::LD_DTV => self.ld_dtv(),
            Opcodes::LD_STV => self.ld_stv(),
            Opcodes::SKP    => self.skp(),
            Opcodes::SKNP   => self.sknp(),
            Opcodes::RND    => self.rnd(),
            Opcodes::SHR    => self.shr(),
            Opcodes::DRW    => self.drw(),
            _               => panic!("Unrecognized opcode: {:?}", self.inst.get_opcode()),
        }

        println!("i: {}", self.i);
        
        if !self.jmp { self.inc_pc(); }
        
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; }
    
        // TODO: refactor this to self.screen.present();
        self.screen.renderer.present();
    }

    fn set_pc(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn inc_pc(&mut self) {
        self.pc += 2;
    }

    fn cls(&mut self) {
        self.screen.clear();
    }

    fn ret(&mut self) {
        let addr = self.stack.pop().unwrap();
        self.set_pc(addr);
    }

    fn jmp(&mut self) {
        let addr: u16 = self.inst.nnn;
        self.set_pc(addr);
        self.jmp = true;
    }
    
    fn jmp_va(&mut self) {
        let offset = self.regs[0] as u16;
        let addr = self.inst.nnn + offset;
        self.set_pc(addr);
        self.jmp = true;
    }

    fn call(&mut self) {
        let old_addr: u16 = self.pc as u16;
        let cur_addr = self.inst.nnn;
        self.stack.push(old_addr);
        self.set_pc(cur_addr);
        self.jmp = true;
    }

    fn se_vb(&mut self) {
        let idx = self.inst.x;
        if self.regs[idx] == self.inst.kk {
            self.inc_pc();
        } 
    }

    fn se_vv(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        if self.regs[idx_x] == self.regs[idx_y] {
            self.inc_pc();
        }
    }

    fn sne_vb(&mut self) {
        let idx = self.inst.x;
        if self.regs[idx] != self.inst.kk {
            self.inc_pc();
        }
    }

    fn sne_vv(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        if self.regs[idx_x] != self.regs[idx_y] {
            self.inc_pc();
        }
    }

    fn or(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        self.regs[idx_x] |= self.regs[idx_y]
    }

    fn and(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        self.regs[idx_x] &= self.regs[idx_y];
    }

    fn add_vb(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        self.regs[idx_x] += self.regs[idx_y];
    }

    fn add_vv(&self) {
        panic!("Not implemented add_vv");
    }

    fn add_iv(&mut self) {
        let idx = self.inst.x;
        self.i += self.regs[idx] as u16;
    }

    fn sub(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        let x = self.regs[idx_x];
        let y = self.regs[idx_y];

        self.regs[15] = if x > y { 1 } else { 0 };
        self.regs[idx_x] -= y;
    }

    fn xor(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        self.regs[idx_x] ^= self.regs[idx_y];
    }

    fn shr(&mut self) {
        let idx_x = self.inst.x;
        let vx    = self.regs[idx_x];
        if vx & 0x01 == 0 {
            self.regs[15] = 1;
        }
        self.regs[idx_x] = vx >> 1;
    }

    fn ld_vv(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        self.regs[idx_x] = self.regs[idx_y];
    }

    fn ld_vb(&mut self) {
        let idx = self.inst.x;
        self.regs[idx] = self.inst.kk;
    }

    fn ld_bv(&mut self) {
        let idx = self.inst.x;
        let mut vx  = self.regs[idx];
        let mut hundred = 100;
        
        for i in 0usize..3 {
            let bcd = vx / hundred;
            self.mem[self.i as usize + i] = bcd;
            vx -= bcd * hundred;
            hundred /= 10;
        }
    }

    fn ld_fv(&mut self) {
        let x = self.regs[self.inst.x];
        self.i = (x * 5) as u16;
    }

    fn ld_vi(&mut self) {
        let x = self.regs[self.inst.x] as usize;
        
        for i in 0usize..x {
           self.regs[i] = self.mem[self.i as usize + i];  
        }
    }

    fn ld_iv(&mut self) {
        let x = self.regs[self.inst.x] as usize;

        for i in 0usize..x {
           self.mem[self.i as usize + i] = self.regs[i]; 
        }
    }
    
    fn ld_ia(&mut self) {
        self.i = self.inst.nnn;
    }
    
    fn ld_vdt(&mut self) {
        let idx = self.inst.x;
        self.regs[idx] = self.dt;
    }
    
    fn ld_dtv(&mut self) {
        let idx: usize = self.inst.x;
        self.dt = self.regs[idx];
    }
    
    fn ld_stv(&mut self) {
        let idx = self.inst.x;
        self.st = self.regs[idx];
    }
   
    fn rnd(&mut self) {
        let idx = self.inst.x;
        let byte = self.inst.kk;
        let n: u8 = thread_rng().gen_range(0,255);
        self.regs[idx] = n & byte;
    }

    fn drw(&mut self) {
        let idx_x = self.inst.x;
        let idx_y = self.inst.y;
        let (x, y) = (self.regs[idx_x] as usize, self.regs[idx_y] as usize);
        let n = self.inst.n as usize;

        let start = self.i;
        
        self.regs[15] = 0;

        for i in 0..n {
            let px = self.mem[(start + i as u16) as usize];
            for j in 0..8 {
                if px & (0x80 >> j) != 0 {
                    if self.screen.buffer[(x+j + (y+i) * 64) as usize] == 1 {
                        self.regs[15] = 1;
                    }               
                    self.screen.buffer[(x+j + (y+i) * 64) as usize] ^= 1;
                }
            } 
        } 
        
        for i in 0usize..320 {
            for j in 0usize..640 {
                if self.screen.buffer[(j/10)+(i/10)*64] == 1 {
                    self.screen.renderer.fill_rect(
                        Rect::new(j as i32, i as i32, 1, 1)
                    );
                }
            }
        }
    }
    
    fn skp(&mut self) {
        let x = self.regs[self.inst.x];
        if x == self.key {
            self.inc_pc();
        }
    }
    
    fn sknp(&mut self) {
        let x = self.regs[self.inst.x];
        if x != self.key {
            self.inc_pc();
        }
    }
}
