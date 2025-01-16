use super::arm7tdmi::ARM7TDMI;

const MUL_MLA_FORMAT: u16 = 0b000_00000_1001;
const MUL_MLA_MASK: u16 = 0b111_11100_1111;

const MULL_MLAL_FORMAT: u16 = 0b000_01000_1001;
const MULL_MLAL_MASK: u16 = 0b111_11000_1111;

const SWP_FORMAT: u16 = 0b000_10000_1001;
const SWP_MASK:u16 = 0b111_11011_1111;

const LDRH_STRH_FORMAT: u16 = 0b000_00000_1011;
const LDRH_STRH_MASK: u16 = 0b111_00000_1111;

const LDRSB_LDRSH_FORMAT: u16 = 0b000_00001_1101;
const LDRSB_LDRSH_MASK: u16 = 0b111_00001_1111;

const MRS_FORMAT: u16 = 0b000_10000_0000;
const MRS_MASK: u16 = 0b111_11011_1111;

const MSR_REG_FORMAT: u16 = 0b000_10010_0000;
const MSR_REG_MASK: u16 = 0b111_11011_1111;

const MSR_IMM_FORMAT: u16 = 0b001_10010_0000;
const MSR_IMM_MASK: u16 = 0b111_11011_0000;

const BX_FORMAT: u16 = 0b000_10010_0001;
const BX_MASK: u16 = 0b111_11111_1111;

const DATAPROC_IMM_SHIFT_FORMAT: u16 = 0b000_00000_0000;
const DATAPROC_IMM_SHIFT_MASK: u16 = 0b111_00000_0001;

const DATAPROC_REG_SHIFT_FORMAT: u16 = 0b000_00000_0001;
const DATAPROC_REG_SHIFT_MASK: u16 = 0b111_00000_1001;

const UNDEF_DATAPROC_FORMART: u16 = 0b001_10000_0000;
const UNDEF_DATAPROC_MASK: u16 = 0b111_11011_0000;

const DATAPROC_IMM_VALUE_FORMAT: u16 = 0b001_00000_0000;
const DATAPROC_IMM_VALUE_MASK: u16 = 0b111_00000_0000;

const LDR_STR_IMM_OFFSET_FORMAT: u16 = 0b010_00000_0000;
const LDR_STR_IMM_OFFSET_MASK: u16 = 0111_00000_0000;

const LDR_STR_REG_OFFSET_FORMAT: u16 = 0b011_00000_0000;
const LDR_STR_REG_OFFSET_MASK: u16 = 0b111_00000_0001;

const LDM_STM_FORMAT: u16 = 0b100_00000_0000;
const LDM_STM_MASK: u16 = 0b111_00000_0000;

const B_BL_FORMAT: u16 = 0b101_00000_0000;
const B_BL_MASK: u16 = 0b111_00000_0000;

const STC_LDC_FORMAT: u16 = 0b110_00000_0000;
const STC_LDC_MASK: u16 = 0b111_00000_0000;

const CDP_FORMAT: u16 = 0b111_00000_0000;
const CDP_MASK: u16 = 0b111_10000_0001;

const MCR_MRC_FORMAT: u16 = 0b111_00000_0001;
const MCR_MRC_MASK: u16 = 0b111_10000_0001;

const SWI_FORMAT: u16 = 0b111_10000_0000;
const SWI_MASK: u16 = 0b111_10000_0000;

#[inline]
pub fn decode_opcode_format_bits(instruction: u32) -> u16 {
    ((instruction >> 16) as u16 & 0x0FF0) | ((instruction >> 4) as u16 & 0xF)
}

#[inline]
pub fn decode_cond_bits(instruction: u32) -> u8 {
    ((instruction >> 28) & 0xF) as u8
}

#[inline]
pub fn decode_dataproc_opcode(instruction: u32) -> u8 {
    ((instruction >> 21) & 0xF) as u8
}

#[inline]
pub fn is_dataproc_immediate(instruction: u32) -> bool {
    ((instruction >> 25) & 1) == 1
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

        assert_eq!(expectec_bits, decode_opcode_format_bits(0x0FF000F0));

        let lo_bits: u8 = 0x8;
        let hi_bits: u8 = 0x88;
        let expectec_bits: u16 = ((hi_bits as u16) << 4) | lo_bits as u16;

        assert_eq!(expectec_bits, decode_opcode_format_bits(0xF88FFF8F));
    }
}