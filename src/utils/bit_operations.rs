pub fn u16_get_bit(number: u16, index: u8) -> bool {
    (number & (1 << index)) != 0
}
