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

fn main() {
    let opt = Opt::from_args();
    let mut emu = Emulator::new();
    if let Err(why) = emu.load(opt.file) {
        return
    }
}

struct Condition_Codes {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8,
}

impl Condition_Codes {
    pub fn new() -> Condition_Codes {
        Condition_Codes {
            z: 1,
            s: 1,
            p: 1,
            cy: 1,
            ac: 1,
            pad: 3,
        }
    }
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
    program_counter: u16,
    memory: Vec<u8>,
    condition_codes: Condition_Codes,
    int_enable: u8,
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
            program_counter: 0,
            memory: Vec::with_capacity(65535),
            condition_codes: Condition_Codes::new(),
            int_enable: 0,
        }
    }

    pub fn load(&mut self, path: PathBuf) -> std::io::Result {
        unimplemented!()
    }

    pub fn run(&mut self) {
        let opcode = self.memory[self.program_counter as usize];
        match opcode {
            0x00 => {}, // NOP
            0x01 => {   // LXI   B,word
                self.c = self.memory[1 + self.program_counter as usize];
                self.b = self.memory[2 + self.program_counter as usize];
                self.program_counter += 2;
            },
            0x041 => self.b = self.c, // MOV B,C
            0x042 => self.b = self.d, // MOV B,D
            0x043 => self.b = self.e, // MOV B,E
            _ => panic!("Unknown opcode {:x}", opcode)
        }

        self.program_counter += 1;
    }
}