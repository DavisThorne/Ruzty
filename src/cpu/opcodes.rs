use super::CPU;

impl CPU {
    pub fn op_null(&mut self) {
        println!("op_null hit");
        //panic!("Unimplemnted Opcode");
    }
    pub fn op_nop(&mut self) {
        println!("op_nop hit");
        //panic!("Stop code hit");
    }
    pub fn op_stop(&mut self) {
        println!("op_stop hit");
        self.stop = true;
    }
    pub fn op_ld_bc_d16(&mut self){
        //let param = ((self.memory[(self.pc+1) as usize] as u16) << 8) | self.memory[(self.pc) as usize] as u16;
        let param = self.fetch_u16() as u16;
        self.bc = param;
    }
    pub fn op_ld_de_d16(&mut self){
        //let param = ((self.memory.memory[(self.pc+1) as usize] as u16) << 8) | self.memory.memory[(self.pc) as usize] as u16;
        let param = self.fetch_u16() as u16;
        self.de = param;
    }
    pub fn build_opcode_table(&mut self){
        self.opcode_table[0x00] = CPU::op_nop;
        self.opcode_table[0x10] = CPU::op_stop;
        self.opcode_table[0x01] = CPU::op_ld_bc_d16;
        self.opcode_table[0x11] = CPU::op_ld_de_d16;
        println!("Opcode table Built")
    }
}



