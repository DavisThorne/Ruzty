use super::CPU;
#[doc(hidden)]
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
    pub fn op_jr_nz_s8(&mut self){
        if (self.af & crate::cpu::flags::Z_FLAG) == 0 {
            self.pc+=0x07; // only jump by 7 because cpu.fetch increments PC by 1
        }
    }
    pub fn op_jr_nc_s8(&mut self){
        if (self.af & crate::cpu::flags::C_FLAG) == 0 {
            self.pc+=0x07;
        }
    }
    pub fn op_ld_b_b(&mut self){
        let data = self.fetch_register_high(self.bc);
        self.bc = (self.bc & 0x00FF) | ((data as u16) << 8);
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
    pub fn op_ld_hl_d16(&mut self){
        let param = self.fetch_u16() as u16;
        self.hl = param;
    }
    pub fn op_ld_sp_d16(&mut self){
        let param= self.fetch_u16() as u16;
        self.sp = param;
    }

    pub fn build_opcode_table(&mut self){
        self.opcode_table[0x00] = CPU::op_nop;
        self.opcode_table[0x10] = CPU::op_stop;
        self.opcode_table[0x20] = CPU::op_jr_nz_s8;
        self.opcode_table[0x30] = CPU::op_jr_nc_s8;
        self.opcode_table[0x01] = CPU::op_ld_bc_d16;
        self.opcode_table[0x11] = CPU::op_ld_de_d16;
        self.opcode_table[0x21] = CPU::op_ld_hl_d16;
        self.opcode_table[0x31] = CPU::op_ld_sp_d16;
        println!("Opcode table Built")
    }
}


