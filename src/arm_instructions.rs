use super::arm7tdmi::ARM7TDMI;

#[inline]
pub fn decode_arm_opcode_bits(instruction: u32) -> u16 {
    ((instruction >> 16) as u16 & 0x0FF0) | ((instruction >> 4) as u16 & 0xF)
}

impl ARM7TDMI {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_decoding_bits_works() {
        let lo_bits: u8 = 0xF;
        let hi_bits: u8 = 0xFF;
        let expectec_bits: u16 = ((hi_bits as u16) << 4) | lo_bits as u16;

        assert_eq!(expectec_bits, decode_arm_opcode_bits(0x0FF000F0));

        let lo_bits: u8 = 0x8;
        let hi_bits: u8 = 0x88;
        let expectec_bits: u16 = ((hi_bits as u16) << 4) | lo_bits as u16;

        assert_eq!(expectec_bits, decode_arm_opcode_bits(0xF88FFF8F));
    }
}