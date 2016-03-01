#![allow(non_camel_case_types)]

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Opcodes {
        NOOP    = 0xFFFF,
        CLS     = 0x00E0,
        RET     = 0x00EE,
        JMP     = 0x1000,
        CALL    = 0x2000,
        SE_VB   = 0x3000,
        SNE_VB  = 0x4000,
        SE_VV   = 0x5000,
        LD_VB   = 0x6000,
        ADD_VB  = 0x7000,
        LD_VV   = 0x8000,
        OR      = 0x8001,
        AND     = 0x8002,
        XOR     = 0x8003,
        ADD_VV  = 0x8004,
        SUB     = 0x8005,
        SHR     = 0x8006,
        SUBN    = 0x8007,
        SHL     = 0x800E,
        SNE_VV  = 0x9000,
        LD_IA   = 0xA000,
        JMP_VA  = 0xB000,
        RND     = 0xC000,
        DRW     = 0xD000,
        SKP     = 0xE09E,
        SKNP    = 0xE0A1,
        LD_VDT  = 0xF007,
        LD_VK   = 0xF00A,
        LD_DTV  = 0xF015,
        LD_STV  = 0xF018,
        ADD_IV  = 0xF01E,
        LD_FV   = 0xF029,
        LD_BV   = 0xF033,
        LD_IV   = 0xF055,
        LD_VI   = 0xF065,
    }
}

#[derive(Default)]
pub struct Instruction {
    pub opcode: u16,
    pub nnn:    u16,
    pub kk:     u8,
    pub x:      usize, 
    pub y:      usize,
    pub n:      u8,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction::default()
    }

    pub fn decode(&mut self, data: (u8,u8)) {
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
}
