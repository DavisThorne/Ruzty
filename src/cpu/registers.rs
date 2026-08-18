#[derive(Debug)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

pub fn decode_r8_src(operand: u8) -> R8 {
    let index = operand & 0b0000_0111;
    match index {
        0 => R8::B,
        1 => R8::C,
        2 => R8::D,
        3 => R8::E,
        4 => R8::H,
        5 => R8::L,
        6 => R8::HL,
        7 => R8::A,
        _ => unreachable!(),
    }
}

pub fn decode_r8_dst(operand: u8) -> R8 {
    let index = (operand & 0b0011_1000) >> 3;
    match index {
        0 => R8::B,
        1 => R8::C,
        2 => R8::D,
        3 => R8::E,
        4 => R8::H,
        5 => R8::L,
        6 => R8::HL,
        7 => R8::A,
        _ => unreachable!(),
    }
}

pub fn decode_r16(operand: u8) -> R16 {
    let index = operand & 0b0011_0000 >> 4;
    match index {
        0 => R16::BC,
        1 => R16::DE,
        2 => R16::HL,
        3 => R16::SP,
        _ => unreachable!(),
    }
}

pub fn fetch_register_high(register: u16) -> u8 {
    let value = (register >> 8) as u8;
    return value;
}

pub fn fetch_register_low(register: u16) -> u8 {
    let value = (register & 0x00FF) as u8;
    return value;
}

pub fn set_register_high(register: &mut u16, data: u8) {
    *register = (*register & 0x00FF) | (data as u16) << 8;
}

pub fn set_register_low(register: &mut u16, data: u8) {
    *register = (*register & 0xFF00) | (data as u16);
}
