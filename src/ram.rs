use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::RAM_SIZE;
use crate::FONTSET;

pub struct RAM{
    ram: [u8; RAM_SIZE],
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            ram: [0; RAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn load_fontset(&mut self) {
        for (i, &byte) in FONTSET.iter().enumerate() {
            self.ram[i] = byte;
        }
    }

    pub fn load_rom(&mut self, rom: &PathBuf) {
        let mut file = File::open(rom).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        for (i, &byte) in buffer.iter().enumerate() {
            self.ram[0x200 + i] = byte;
        }
    }
}
