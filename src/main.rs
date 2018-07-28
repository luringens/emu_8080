#![feature(nll)]
#![allow(dead_code)]

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let mut emu = Emulator::new();
    emu.load(opt.file)?;
    emu.run();
    Ok(())
}

struct ConditionCodes {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    // pub aux_carry: u8, NYI
}

impl ConditionCodes {
    pub fn new() -> ConditionCodes {
        ConditionCodes {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
        }
    }
}

enum Status {
    Standby,
    Running,
    Halt,
    Error(String),
}

struct Emulator {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Vec<u8>,
    /// Program counter
    condition_codes: ConditionCodes,
    int_enable: u8,
    status: Status,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: Vec::with_capacity(65535),
            condition_codes: ConditionCodes::new(),
            int_enable: 0,
            status: Status::Standby,
        }
    }

    pub fn load(&mut self, path: PathBuf) -> std::io::Result<()> {
        unimplemented!()
    }

    pub fn run(&mut self) {
        let mut result = Ok(());
        while result.is_ok() {
            result = self.step();
        }
    }

    pub fn step(&mut self) {
        let opcode = self.memory[self.pc as usize];
        self.status = Status::Running;
        match opcode {
            0x00 => {}, // NOP

            // Data transfer.
            0x01 => { // LXI   B,word
                self.c = self.memory[1 + self.pc as usize];
                self.b = self.memory[2 + self.pc as usize];
                self.pc += 2;
            },            
            0x02 => unimplemented!(), //STAX B
            0x03 => unimplemented!(), //INX B
            0x04 => unimplemented!(), //INR B
            0x05 => unimplemented!(), //DCR B
            0x06 => unimplemented!(), //MVI B, D8
            0x07 => unimplemented!(), //RLC
            
            0x09 => unimplemented!(), //DAD B
            0x0A => unimplemented!(), //LDAX B
            0x0B => unimplemented!(), //DCX B
            0x0C => unimplemented!(), //INR C
            0x0D => unimplemented!(), //DCR C
            0x0E => unimplemented!(), //MVI C,D8
            0x0F => unimplemented!(), //RRC
            
            0x11 => unimplemented!(), //LXI D,D16
            0x12 => unimplemented!(), //STAX D
            0x13 => unimplemented!(), //INX D
            0x14 => unimplemented!(), //INR D
            0x15 => unimplemented!(), //DCR D
            0x16 => unimplemented!(), //MVI D, D8
            0x17 => unimplemented!(), //RAL
            
            0x19 => unimplemented!(), //DAD D
            0x1A => unimplemented!(), //LDAX D
            0x1B => unimplemented!(), //DCX D
            0x1C => unimplemented!(), //INR E
            0x1D => unimplemented!(), //DCR E
            0x1E => unimplemented!(), //MVI E,D8
            0x1F => unimplemented!(), //RAR
            0x20 => unimplemented!(), //RIM
            0x21 => unimplemented!(), //LXI H,D16
            0x22 => unimplemented!(), //SHLD adr
            0x23 => unimplemented!(), //INX H
            0x24 => unimplemented!(), //INR H
            0x25 => unimplemented!(), //DCR H
            0x26 => unimplemented!(), //MVI H,D8
            0x27 => unimplemented!(), //DAA
            
            0x29 => unimplemented!(), //DAD H
            0x2A => unimplemented!(), //LHLD adr
            0x2B => unimplemented!(), //DCX H
            0x2C => unimplemented!(), //INR L
            0x2D => unimplemented!(), //DCR L
            0x2E => unimplemented!(), //MVI L, D8
            0x2F => unimplemented!(), //CMA
            0x30 => unimplemented!(), //SIM
            0x31 => unimplemented!(), //LXI SP, D16
            0x32 => unimplemented!(), //STA adr
            0x33 => unimplemented!(), //INX SP
            0x34 => unimplemented!(), //INR M
            0x35 => unimplemented!(), //DCR M
            0x36 => unimplemented!(), //MVI M,D8
            0x37 => unimplemented!(), //STC
            
            0x39 => unimplemented!(), //DAD SP
            0x3A => unimplemented!(), //LDA adr
            0x3B => unimplemented!(), //DCX SP
            0x3C => unimplemented!(), //INR A
            0x3D => unimplemented!(), //DCR A
            0x3E => unimplemented!(), //MVI A,D8
            0x3F => unimplemented!(), //CMC
            0x40 => {}, //MOV B,B
            0x41 => self.b = self.c, //MOV B,C
            0x42 => self.b = self.d, //MOV B,D
            0x43 => self.b = self.e, //MOV B,E
            0x44 => self.b = self.h, //MOV B,H
            0x45 => self.b = self.l, //MOV B,L
            0x46 => unimplemented!(), //MOV B,M
            0x47 => unimplemented!(), //MOV B,A
            0x48 => unimplemented!(), //MOV C,B
            0x49 => {}, //MOV C,C
            0x4A => unimplemented!(), //MOV C,D
            0x4B => unimplemented!(), //MOV C,E
            0x4C => unimplemented!(), //MOV C,H
            0x4D => unimplemented!(), //MOV C,L
            0x4E => unimplemented!(), //MOV C,M
            0x4F => unimplemented!(), //MOV C,A
            0x50 => unimplemented!(), //MOV D,B
            0x51 => unimplemented!(), //MOV D,C
            0x52 => {}, //MOV D,D
            0x53 => unimplemented!(), //MOV D,E
            0x54 => unimplemented!(), //MOV D,H
            0x55 => unimplemented!(), //MOV D,L
            0x56 => unimplemented!(), //MOV D,M
            0x57 => unimplemented!(), //MOV D,A
            0x58 => unimplemented!(), //MOV E,B
            0x59 => unimplemented!(), //MOV E,C
            0x5A => unimplemented!(), //MOV E,D
            0x5B => {}, //MOV E,E
            0x5C => unimplemented!(), //MOV E,H
            0x5D => unimplemented!(), //MOV E,L
            0x5E => unimplemented!(), //MOV E,M
            0x5F => unimplemented!(), //MOV E,A
            0x60 => unimplemented!(), //MOV H,B
            0x61 => unimplemented!(), //MOV H,C
            0x62 => unimplemented!(), //MOV H,D
            0x63 => unimplemented!(), //MOV H,E
            0x64 => {}, //MOV H,H
            0x65 => unimplemented!(), //MOV H,L
            0x66 => unimplemented!(), //MOV H,M
            0x67 => unimplemented!(), //MOV H,A
            0x68 => unimplemented!(), //MOV L,B
            0x69 => unimplemented!(), //MOV L,C
            0x6A => unimplemented!(), //MOV L,D
            0x6B => unimplemented!(), //MOV L,E
            0x6C => unimplemented!(), //MOV L,H
            0x6D => {}, //MOV L,L
            0x6E => unimplemented!(), //MOV L,M
            0x6F => unimplemented!(), //MOV L,A
            0x70 => unimplemented!(), //MOV M,B
            0x71 => unimplemented!(), //MOV M,C
            0x72 => unimplemented!(), //MOV M,D
            0x73 => unimplemented!(), //MOV M,E
            0x74 => unimplemented!(), //MOV M,H
            0x75 => unimplemented!(), //MOV M,L
            0x76 => { //HLT
                self.status = Status::Halt;
                return
            }, 
            0x77 => unimplemented!(), //MOV M,A
            0x78 => unimplemented!(), //MOV A,B
            0x79 => unimplemented!(), //MOV A,C
            0x7A => unimplemented!(), //MOV A,D
            0x7B => unimplemented!(), //MOV A,E
            0x7C => unimplemented!(), //MOV A,H
            0x7D => unimplemented!(), //MOV A,L
            0x7E => unimplemented!(), //MOV A,M
            0x7F => {}, //MOV A,A
            
            // Arithmetic
            0x80 => self.add(self.a, self.b), // ADD B
            0x81 => self.add(self.a, self.c), // ADD C
            0x82 => self.add(self.a, self.d), // ADD D
            0x83 => self.add(self.a, self.e), // ADD E
            0x84 => self.add(self.a, self.h), // ADD H
            0x85 => self.add(self.a, self.l), // ADD L
            0x86 => self.add(self.a, self.memory[self.get_hl_offset()]), //ADD M
            0x87 => self.add(self.a, self.a), // ADD A
            0x88 => unimplemented!(), //ADC B
            0x89 => unimplemented!(), //ADC C
            0x8A => unimplemented!(), //ADC D
            0x8B => unimplemented!(), //ADC E
            0x8C => unimplemented!(), //ADC H
            0x8D => unimplemented!(), //ADC L
            0x8E => unimplemented!(), //ADC M
            0x8F => unimplemented!(), //ADC A
            0x90 => unimplemented!(), //SUB B
            0x91 => unimplemented!(), //SUB C
            0x92 => unimplemented!(), //SUB D
            0x93 => unimplemented!(), //SUB E
            0x94 => unimplemented!(), //SUB H
            0x95 => unimplemented!(), //SUB L
            0x96 => unimplemented!(), //SUB M
            0x97 => unimplemented!(), //SUB A
            0x98 => unimplemented!(), //SBB B
            0x99 => unimplemented!(), //SBB C
            0x9A => unimplemented!(), //SBB D
            0x9B => unimplemented!(), //SBB E
            0x9C => unimplemented!(), //SBB H
            0x9D => unimplemented!(), //SBB L
            0x9E => unimplemented!(), //SBB M
            0x9F => unimplemented!(), //SBB A
            0xA0 => unimplemented!(), //ANA B
            0xA1 => unimplemented!(), //ANA C
            0xA2 => unimplemented!(), //ANA D
            0xA3 => unimplemented!(), //ANA E
            0xA4 => unimplemented!(), //ANA H
            0xA5 => unimplemented!(), //ANA L
            0xA6 => unimplemented!(), //ANA M
            0xA7 => unimplemented!(), //ANA A
            0xA8 => unimplemented!(), //XRA B
            0xA9 => unimplemented!(), //XRA C
            0xAA => unimplemented!(), //XRA D
            0xAB => unimplemented!(), //XRA E
            0xAC => unimplemented!(), //XRA H
            0xAD => unimplemented!(), //XRA L
            0xAE => unimplemented!(), //XRA M
            0xAF => unimplemented!(), //XRA A
            0xB0 => unimplemented!(), //ORA B
            0xB1 => unimplemented!(), //ORA C
            0xB2 => unimplemented!(), //ORA D
            0xB3 => unimplemented!(), //ORA E
            0xB4 => unimplemented!(), //ORA H
            0xB5 => unimplemented!(), //ORA L
            0xB6 => unimplemented!(), //ORA M
            0xB7 => unimplemented!(), //ORA A
            0xB8 => unimplemented!(), //CMP B
            0xB9 => unimplemented!(), //CMP C
            0xBA => unimplemented!(), //CMP D
            0xBB => unimplemented!(), //CMP E
            0xBC => unimplemented!(), //CMP H
            0xBD => unimplemented!(), //CMP L
            0xBE => unimplemented!(), //CMP M
            0xBF => unimplemented!(), //CMP A
            0xC0 => unimplemented!(), //RNZ
            0xC1 => unimplemented!(), //POP B
            0xC2 => unimplemented!(), //JNZ adr
            0xC3 => unimplemented!(), //JMP adr
            0xC4 => unimplemented!(), //CNZ adr
            0xC5 => unimplemented!(), //PUSH B
            0xC6 => self.add(self.a, self.memory[1 + self.pc as usize]), //ADI D8
            0xC7 => unimplemented!(), //RST 0
            0xC8 => unimplemented!(), //RZ
            0xC9 => unimplemented!(), //RET
            0xCA => unimplemented!(), //JZ adr
            
            0xCC => unimplemented!(), //CZ adr
            0xCD => unimplemented!(), //CALL adr
            0xCE => unimplemented!(), //ACI D8
            0xCF => unimplemented!(), //RST 1
            0xD0 => unimplemented!(), //RNC
            0xD1 => unimplemented!(), //POP D
            0xD2 => unimplemented!(), //JNC adr
            0xD3 => unimplemented!(), //OUT D8
            0xD4 => unimplemented!(), //CNC adr
            0xD5 => unimplemented!(), //PUSH D
            0xD6 => unimplemented!(), //SUI D8
            0xD7 => unimplemented!(), //RST 2
            0xD8 => unimplemented!(), //RC
            
            0xDA => unimplemented!(), //JC adr
            0xDB => unimplemented!(), //IN D8
            0xDC => unimplemented!(), //CC adr
            
            0xDE => unimplemented!(), //SBI D8
            0xDF => unimplemented!(), //RST 3
            0xE0 => unimplemented!(), //RPO
            0xE1 => unimplemented!(), //POP H
            0xE2 => unimplemented!(), //JPO adr
            0xE3 => unimplemented!(), //XTHL
            0xE4 => unimplemented!(), //CPO adr
            0xE5 => unimplemented!(), //PUSH H
            0xE6 => unimplemented!(), //ANI D8
            0xE7 => unimplemented!(), //RST 4
            0xE8 => unimplemented!(), //RPE
            0xE9 => unimplemented!(), //PCHL
            0xEA => unimplemented!(), //JPE adr
            0xEB => unimplemented!(), //XCHG
            0xEC => unimplemented!(), //CPE adr
            
            0xEE => unimplemented!(), //XRI D8
            0xEF => unimplemented!(), //RST 5
            0xF0 => unimplemented!(), //RP
            0xF1 => unimplemented!(), //POP PSW
            0xF2 => unimplemented!(), //JP adr
            0xF3 => unimplemented!(), //DI
            0xF4 => unimplemented!(), //CP adr
            0xF5 => unimplemented!(), //PUSH PSW
            0xF6 => unimplemented!(), //ORI D8
            0xF7 => unimplemented!(), //RST 6
            0xF8 => unimplemented!(), //RM
            0xF9 => unimplemented!(), //SPHL
            0xFA => unimplemented!(), //JM adr
            0xFB => unimplemented!(), //EI
            0xFC => unimplemented!(), //CM adr
            
            0xFE => unimplemented!(), //CPI D8
            0xFF => unimplemented!(), //RST 7

            _ => {
                self.status = Status::Error(format!("Unknown opcode: {:x}", opcode));
                return
            }
        }

        self.pc += 1;
    }

    fn get_hl_offset(&self) -> usize {
        ((self.h as u16) << 8 | self.l as u16) as usize
    }

    fn add(&mut self, fst: u8, snd: u8) {
        let (ans, overflow) = fst.overflowing_add(snd);
        self.a = ans;
        self.condition_codes.carry = overflow;
        self.update_condition();
    }
    
    fn sub(&mut self, fst: u8, snd: u8) {
        let (ans, overflow) = fst.overflowing_sub(snd);
        self.a = ans;
        self.condition_codes.carry = overflow;
        self.update_condition();
    }

    fn update_condition(&mut self) {
        self.condition_codes.zero = self.a == 0;
        self.condition_codes.sign = self.a & 0x80 == 1;
        self.condition_codes.parity = parity(self.a);
    }    
}

fn parity(num: u8) -> bool {
    let mut count: u8 = 0;
    for i in 0..8 {
        if num & (1 << i) != 0 { count += 1 };
    }
    count % 2 == 0
}
