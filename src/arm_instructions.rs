use crate::system_memory::SysMem;

use super::arm7tdmi::ARM7TDMI;

// enum ARMInstructionType {
//     MUL_MLA(fn(u32)),
//     MULL_MLAL(fn(u32)),
//     SWP(fn(u32)),
//     LDRH_STRH(fn(u32)),
//     LDRSB_LDRSH(fn(u32)),
//     MRS(fn(u32)),
//     MSR_REG(fn(u32)),
//     MSR_IMM(fn(u32)),
//     BX(fn(u32)),
//     DATAPROC_IMM_SHIFT(fn(u32)),
//     DATAPROC_REG_SHIFT(fn(u32)),
//     UNDEF_DATAPROC(fn(u32)),
//     DATAPROC_IMM_VALUE(fn(u32)),
//     LDR_STR_IMM_OFFSET(fn(u32)),
//     LDR_STR_REG_OFFSET(fn(u32)),
//     LDM_STM(fn(u32)),
//     B_BL(fn(u32)),
//     STC_LDC(fn(u32)),
//     CDP(fn(u32)),
//     MCR_MRC(fn(u32)),
//     SWI(fn(u32))
// }

#[inline]
pub fn arm_decode_opcode_format_bits(instruction: u32) -> u16 {
    ((instruction >> 16) as u16 & 0x0FF0) | ((instruction >> 4) as u16 & 0xF)
}

#[inline]
pub fn arm_decode_cond_bits(instruction: u32) -> u8 {
    ((instruction >> 28) & 0xF) as u8
}

#[inline]
pub fn arm_decode_dataproc_opcode(instruction: u32) -> u8 {
    ((instruction >> 21) & 0xF) as u8
}

#[inline]
pub fn arm_is_dataproc_immediate(instruction: u32) -> bool {
    ((instruction >> 25) & 1) == 1
}

#[inline]
pub fn is_mul_mla_inst(instruction: u32) -> bool {
    const MUL_MLA_FORMAT: u16 = 0b000_00000_1001;
    const MUL_MLA_MASK: u16 = 0b111_11100_1111;

    return (arm_decode_opcode_format_bits(instruction) & MUL_MLA_MASK) == MUL_MLA_FORMAT;
}

#[inline]
pub fn is_mull_mlal_inst(instruction: u32) -> bool {
    const MULL_MLAL_FORMAT: u16 = 0b000_01000_1001;
    const MULL_MLAL_MASK: u16 = 0b111_11000_1111;

    return (arm_decode_opcode_format_bits(instruction) & MULL_MLAL_MASK) == MULL_MLAL_FORMAT;
}

#[inline]
pub fn is_swap_inst(instruction: u32) -> bool {
    const SWP_FORMAT: u16 = 0b000_10000_1001;
    const SWP_MASK:u16 = 0b111_11011_1111;

    return (arm_decode_opcode_format_bits(instruction) & SWP_MASK) == SWP_FORMAT;
}

#[inline]
pub fn is_ldrh_strh_inst(instruction: u32) -> bool {
    const LDRH_STRH_FORMAT: u16 = 0b000_00000_1011;
    const LDRH_STRH_MASK: u16 = 0b111_00000_1111;

    return (arm_decode_opcode_format_bits(instruction) & LDRH_STRH_MASK) == LDRH_STRH_FORMAT;
}

#[inline]
pub fn is_ldrsb_strsh_inst(instruction: u32) -> bool {
    const LDRSB_LDRSH_FORMAT: u16 = 0b000_00001_1101;
    const LDRSB_LDRSH_MASK: u16 = 0b111_00001_1111;
    
    return (arm_decode_opcode_format_bits(instruction) & LDRSB_LDRSH_MASK) == LDRSB_LDRSH_FORMAT;
}

#[inline]
pub fn is_mrs_inst(instruction: u32) -> bool {
    const MRS_FORMAT: u16 = 0b000_10000_0000;
    const MRS_MASK: u16 = 0b111_11011_1111;

    return (arm_decode_opcode_format_bits(instruction) & MRS_MASK) == MRS_FORMAT;
}

#[inline]
pub fn is_msr_reg_inst(instruction: u32) -> bool {
    const MSR_REG_FORMAT: u16 = 0b000_10010_0000;
    const MSR_REG_MASK: u16 = 0b111_11011_1111;

    return (arm_decode_opcode_format_bits(instruction) & MSR_REG_MASK) == MSR_REG_FORMAT;
}

#[inline]
pub fn is_msr_imm_inst(instruction: u32) -> bool {
    const MSR_IMM_FORMAT: u16 = 0b001_10010_0000;
    const MSR_IMM_MASK: u16 = 0b111_11011_0000;

    return (arm_decode_opcode_format_bits(instruction) & MSR_IMM_MASK) == MSR_IMM_FORMAT;
}

#[inline]
pub fn is_bx_inst(instruction: u32) -> bool {
    const BX_FORMAT: u16 = 0b000_10010_0001;
    const BX_MASK: u16 = 0b111_11111_1111;

    return (arm_decode_opcode_format_bits(instruction) & BX_MASK) == BX_FORMAT;
}

#[inline]
pub fn is_dataproc_imm_shift_inst(instruction: u32) -> bool {   
    const DATAPROC_IMM_SHIFT_FORMAT: u16 = 0b000_00000_0000;
    const DATAPROC_IMM_SHIFT_MASK: u16 = 0b111_00000_0001;

    return (arm_decode_opcode_format_bits(instruction) & DATAPROC_IMM_SHIFT_MASK) == DATAPROC_IMM_SHIFT_FORMAT;
}

#[inline]
pub fn is_dataproc_reg_shift_inst(instruction: u32) -> bool {   
    const DATAPROC_REG_SHIFT_FORMAT: u16 = 0b000_00000_0001;
    const DATAPROC_REG_SHIFT_MASK: u16 = 0b111_00000_1001;

    return (arm_decode_opcode_format_bits(instruction) & DATAPROC_REG_SHIFT_MASK) == DATAPROC_REG_SHIFT_FORMAT;
}

#[inline]
pub fn is_undef_dataproc_inst(instruction: u32) -> bool {
    const UNDEF_DATAPROC_FORMAT: u16 = 0b001_10000_0000;
    const UNDEF_DATAPROC_MASK: u16 = 0b111_11011_0000;

    return (arm_decode_opcode_format_bits(instruction) & UNDEF_DATAPROC_MASK) == UNDEF_DATAPROC_FORMAT;
}

#[inline]
pub fn is_dataproc_imm_value_inst(instruction: u32) -> bool {
    const DATAPROC_IMM_VALUE_FORMAT: u16 = 0b001_00000_0000;
    const DATAPROC_IMM_VALUE_MASK: u16 = 0b111_00000_0000;

    return (arm_decode_opcode_format_bits(instruction) & DATAPROC_IMM_VALUE_MASK) == DATAPROC_IMM_VALUE_FORMAT;
}

#[inline]
pub fn is_ldr_str_imm_offset_inst(instruction: u32) -> bool {
    const LDR_STR_IMM_OFFSET_FORMAT: u16 = 0b010_00000_0000;
    const LDR_STR_IMM_OFFSET_MASK: u16 = 0b111_00000_0000;

    return (arm_decode_opcode_format_bits(instruction) & LDR_STR_IMM_OFFSET_MASK) == LDR_STR_IMM_OFFSET_FORMAT;
}

#[inline]
pub fn is_ldr_str_reg_offset_inst(instruction: u32) -> bool {
    const LDR_STR_REG_OFFSET_FORMAT: u16 = 0b011_00000_0000;
    const LDR_STR_REG_OFFSET_MASK: u16 = 0b111_00000_0001;

    return (arm_decode_opcode_format_bits(instruction) & LDR_STR_REG_OFFSET_MASK) == LDR_STR_REG_OFFSET_FORMAT;
}

#[inline]
pub fn is_ldm_stm_inst(instruction: u32) -> bool {
    const LDM_STM_FORMAT: u16 = 0b100_00000_0000;
    const LDM_STM_MASK: u16 = 0b111_00000_0000;

    return (arm_decode_opcode_format_bits(instruction) & LDM_STM_MASK) == LDM_STM_FORMAT;
}

#[inline]
pub fn is_b_bl_inst(instruction: u32) -> bool {
    const B_BL_FORMAT: u16 = 0b101_00000_0000;
    const B_BL_MASK: u16 = 0b111_00000_0000;

    return (arm_decode_opcode_format_bits(instruction) & B_BL_MASK) == B_BL_FORMAT;
}

#[inline]
pub fn is_stc_ldc_inst(instruction: u32) -> bool {
    const STC_LDC_FORMAT: u16 = 0b110_00000_0000;
    const STC_LDC_MASK: u16 = 0b111_00000_0000;

    return (arm_decode_opcode_format_bits(instruction) & STC_LDC_MASK) == STC_LDC_FORMAT;
}

#[inline]
pub fn is_cdp_inst(instruction: u32) -> bool {
    const CDP_FORMAT: u16 = 0b111_00000_0000;
    const CDP_MASK: u16 = 0b111_10000_0001;

    return (arm_decode_opcode_format_bits(instruction) & CDP_MASK) == CDP_FORMAT;
}

#[inline]
pub fn is_mcr_mrc_inst(instruction: u32) -> bool {
    const MCR_MRC_FORMAT: u16 = 0b111_00000_0001;
    const MCR_MRC_MASK: u16 = 0b111_10000_0001;

    return (arm_decode_opcode_format_bits(instruction) & MCR_MRC_MASK) == MCR_MRC_FORMAT;
}

#[inline]
pub fn is_swi_inst(instruction: u32) -> bool {
    const SWI_FORMAT: u16 = 0b111_10000_0000;
    const SWI_MASK: u16 = 0b111_10000_0000;

    return (arm_decode_opcode_format_bits(instruction) & SWI_MASK) == SWI_FORMAT;
}

pub fn nop(_: u32, _: &mut SysMem) {
}

impl ARM7TDMI {
    pub fn decode_arm_instruction(&self, instruction: u32) -> fn(u32, &mut SysMem) {
        // Decoding order of instructions

        if is_mul_mla_inst(instruction) {
            return nop;
        }
        
        if is_mull_mlal_inst(instruction) {
            return nop;
        }

        if is_swap_inst(instruction) {
            return nop;
        }

        if is_ldrh_strh_inst(instruction) {
            return nop;
        }

        if is_ldrsb_strsh_inst(instruction) {
            return nop;
        }

        if is_mrs_inst(instruction) {
            return nop;
        }

        if is_msr_reg_inst(instruction) {
            return nop;
        }

        if is_msr_imm_inst(instruction) {
            return nop;
        }

        if is_bx_inst(instruction) {
            return nop;
        }

        if is_dataproc_imm_shift_inst(instruction) {
            return nop;
        }

        if is_dataproc_reg_shift_inst(instruction) {
            return nop;
        }

        if is_undef_dataproc_inst(instruction) {
            return nop;
        }

        if is_dataproc_imm_value_inst(instruction) {
            return nop;
        }

        if is_ldr_str_imm_offset_inst(instruction) {
            return nop;
        }

        if is_ldr_str_reg_offset_inst(instruction) {
            return nop;
        }

        if is_ldm_stm_inst(instruction) {
            return nop;
        }

        if is_b_bl_inst(instruction) {
            return nop;
        }

        if is_stc_ldc_inst(instruction) {
            return nop;
        }

        if is_cdp_inst(instruction) {
            return nop;
        }

        if is_mcr_mrc_inst(instruction) {
            return nop;
        }

        if is_swi_inst(instruction) {
            return nop;
        }

        // Default
        nop
    }

    fn mul_mla(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn mull_mlal(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn swap(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn ldrh_strh(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn ldrsb_ldrsh(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn mrs(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn msr_reg(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn msr_imm(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn bx(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn dataproc_imm_shift(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn dataproc_reg_shift(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn undef_dataproc(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn dataproc_imm_value(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn ldr_str_imm_offset(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn ldr_str_reg_offset(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn ldm_stm(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn b_bl(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn stc_ldc(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn cdp(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn mcr_mrc(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }

    fn swi(&mut self, instruction: u32, sys_mem: &mut SysMem) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_decoding_bits_works() {
        let lo_bits: u8 = 0xF;
        let hi_bits: u8 = 0xFF;
        let expectec_bits: u16 = ((hi_bits as u16) << 4) | lo_bits as u16;

        assert_eq!(expectec_bits, arm_decode_opcode_format_bits(0x0FF000F0));

        let lo_bits: u8 = 0x8;
        let hi_bits: u8 = 0x88;
        let expectec_bits: u16 = ((hi_bits as u16) << 4) | lo_bits as u16;

        assert_eq!(expectec_bits, arm_decode_opcode_format_bits(0xF88FFF8F));
    }
}