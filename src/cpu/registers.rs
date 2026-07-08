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
