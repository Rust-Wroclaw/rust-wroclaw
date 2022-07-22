const TWO: i32 = 2;
const THREE: i32 = 3;

// As advertised, triggers on this kind of incorrect code
pub fn check_flags(x: i32) -> bool {
    2 & x == 3
}

// Also works with constants!
pub fn check_flags_with_consts(x: i32) -> bool {
    TWO & x == THREE
}

// The cases become obvious when you look at the binary representation
pub fn some_examples(x: i32) -> bool {
    x & 0b0101 > 0b0101 // x & 0101 will never be larger than 0101
    && x & 0b0101 == 0b0110 // x & 0101 will never equal 0110
    && x | 0b01 == 0b00 // x | 01 will never be equal to 00
    && x | 0b0110 < 0b0010 // x | 0110 will never be smaller than 0010
}
