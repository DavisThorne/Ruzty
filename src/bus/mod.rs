use crate::memory::MEMORY;

pub struct BUS {
    memory: MEMORY
}

impl BUS {
    pub fn new(memory: MEMORY) -> Self {
        BUS {
            memory: memory
        }
    }
    pub fn read(&mut self, addr: u16) -> u8 {
        return self.memory.read_mem(addr);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory.write_mem(addr, data);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.load(program);
    }
}