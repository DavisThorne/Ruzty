pub struct MEMORY {
    memory: [u8; 0xFFFF]
}

impl MEMORY {
    pub fn new() -> Self {
        MEMORY{
            memory: [0x00; 0xFFFF]
        }
    }

    pub fn write_mem(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read_mem(&mut self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x0000 .. (0x0000 + program.len())].copy_from_slice(&program[..]);
        //self.pc = 0x0000
    }
}