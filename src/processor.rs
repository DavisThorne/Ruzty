pub struct CPU {
    PC: u16,    //Program Counter
    AF: u16,    //Accumulator&Flags Hi=Accumulator Lo=Flags
    BC: u16,    //B&C Hi=B Lo=C
    DE: u16,    //D&E Hi=D Lo=E
    HL: u16,    //H&L Hi=H Lo=L
    SP: u16     //Stack Pointer
}


impl CPU {
    pub fn new() -> Self {
        CPU {
            PC: 0,
            AF: 0,
            BC: 0,
            DE: 0,
            HL: 0,
            SP: 0
        }
    }

    pub fn interpreter(&mut self, program: Vec<u8>){
        self.PC = 0;
        loop {
            let opcode = program[self.PC as usize] as u16;
            self.PC += 1;

            match opcode {
                0x00 => { //NOP
                    self.PC += 1;
                },
                0x1000 => { //STOP
                    return;
                },
                0x01 => { //LD BC, d16
                    let param = ((program[self.PC+1 as usize] as u16) << 8) | program[(self.PC) as usize] as u16;
                    self.BC = param;
                },
                _ => {todo!();}
            }
        }
    }
}