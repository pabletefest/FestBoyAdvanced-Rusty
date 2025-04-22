use crate::{arm_instructions::arm_decode_cond_bits, system_memory::{MemoryOperation, SysMem}};

const SP: usize = 13;
const LR: usize = 14;
const PC: usize = 15;

const MODE_BITS_MASK: u32 = 0x0000001F;

const EXCEPTIONS_HANDLERS_ADDRESSES: [u32; 8] = [0x00000000, 0x00000004, 0x00000008, 0x0000000C, 0x00000010, 0x00000014, 0x00000018, 0x0000001C];

#[derive(Clone, Copy, Eq, PartialEq)]
enum CpuStateMode {
    ARM = 0,
    THUMB = 1
}

#[derive(Clone, Copy)]
enum OperationModes {
    User = 16,
    FIQ = 17,
    IRQ = 18,
    Supervisor = 19,
    Abort = 23,
    Undefined = 27,
    System = 31
}

enum CPSRBitsMask {
    N = 0x80000000,
    Z = 0x40000000,
    C = 0x20000000,
    V = 0x10000000,
    I = 0x00000080,
    F = 0x00000040,
    T = 0x00000020
}

#[derive(Clone, Copy)]
enum ExceptionType {
    Reset,
    UndefinedInstruction,
    SoftwareInterrupt,
    PrefetchAbort,
    DataAbort,
    AddressExceeds,
    NormalInterrupt,
    FastInterrupt
}

// #[derive(Clone, Copy)]
// enum RegsBank {
//     BankUsrSys = 0,
//     BankFiq = 1,
//     BankIrq = 2,
//     BankSvs = 3,
//     BankAbt = 4,
//     BankUnd = 5
// }

pub struct ARM7TDMI {
    gpr: [u32; 16],
    banked_user_sys_regs: [u32; 16],
    banked_fiq_regs: [u32; 7],
    banked_svc_regs: [u32; 2],
    banked_abt_regs: [u32; 2],
    banked_irq_regs: [u32; 2],
    banked_und_regs: [u32; 2],
    cpsr: u32,
    spsr_user_sys: u32,
    spsr_fiq: u32,
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_irq: u32,
    spsr_und: u32,

    cpu_mode: CpuStateMode,
    operation_mode: OperationModes,

    pipeline: [Option<u32>; 2],

    instruction_cycles: u32
}

impl ARM7TDMI {
    pub fn new() -> Self {
        ARM7TDMI {
            gpr: [0; 16],
            banked_user_sys_regs: [0; 16],
            banked_fiq_regs: [0; 7],
            banked_svc_regs: [0; 2],
            banked_abt_regs: [0; 2],
            banked_irq_regs: [0; 2],
            banked_und_regs: [0; 2],
            cpsr: 0u32,
            spsr_user_sys: 0u32,
            spsr_fiq: 0u32,
            spsr_svc: 0u32,
            spsr_abt: 0u32,
            spsr_irq: 0u32,
            spsr_und: 0u32,
            pipeline: [None; 2],
            cpu_mode: CpuStateMode::ARM,
            operation_mode: OperationModes::User,
            instruction_cycles: 0u32
        }
    }
    
    pub fn reset(&mut self, sys_mem: &mut SysMem) {
        self.flush_pipeline(sys_mem);
    }

    fn flush_pipeline(&mut self, sys_mem: &mut SysMem) {
        if self.cpu_mode == CpuStateMode::ARM {
            self.pipeline[0] = Some(sys_mem.read32(self.pc() as usize));
            self.increment_pc();
            self.pipeline[1] = Some(sys_mem.read32(self.pc() as usize));
            self.increment_pc();
        }
        else {
            self.pipeline[0] = Some(sys_mem.read16(self.pc() as usize) as u32);
            self.increment_pc();
            self.pipeline[1] = Some(sys_mem.read16(self.pc() as usize) as u32);
            self.increment_pc();
        }
    }

    pub fn run_instruction(&mut self, sys_mem: &mut SysMem) -> u8 {
        let opcode: u32 = self.pipeline[0].unwrap();
        self.pipeline.rotate_left(1);

        if self.cpu_mode == CpuStateMode::ARM {
            self.pipeline[1] = Some(sys_mem.read32(self.pc() as usize));
            self.increment_pc();

            if arm_decode_cond_bits(opcode) > 0 { // Execute this instruction
                let instruction_ptr = self.decode_arm_instruction(opcode);
                instruction_ptr(opcode, sys_mem);
            }
        }
        else {
            self.pipeline[1] = Some(sys_mem.read16(self.pc() as usize) as u32);
            self.increment_pc();

            // TODO: Thumb Mode
        }

        // TODO: Return cycles needed for the executed instruction
        0
    }

    fn pc(&self) -> u32 {
       self.gpr[PC]
    }

    fn pc_mut(&mut self, value: u32) {
        self.gpr[PC] = value;
    }

    fn increment_pc(&mut self) {
        if self.cpu_mode == CpuStateMode::ARM {
            self.gpr[PC] = self.gpr[PC].wrapping_add(4);
        }
        else {
            self.gpr[PC] = self.gpr[PC].wrapping_add(2);
        }
    }

    fn set_cpsr_bit(&mut self, bit_mask: CPSRBitsMask) {
        self.cpsr |= bit_mask as u32;
    }

    fn clear_cpsr_bit(&mut self, bit_mask: CPSRBitsMask) {
        self.cpsr &= !(bit_mask as u32);
    }

    fn get_cpsr_bit(self, bit_mask: CPSRBitsMask) -> bool {
        (self.cpsr & bit_mask as u32) > 0 
    }

    fn enter_operation_mode(&mut self, new_mode: OperationModes) {    
        let prev_mode = self.operation_mode;
        
        // self.cpsr = (self.cpsr & 0x0000001F) | mode as u32;
        self.cpsr = (self.cpsr & 0xFFFFFFE0) | new_mode as u32;
        self.operation_mode = new_mode;
        
        match prev_mode {
            // OperationModes::User => {},
            OperationModes::FIQ => {
                self.spsr_fiq = self.cpsr;
                self.banked_fiq_regs.copy_from_slice(&self.gpr[8..PC]);
            },
            OperationModes::IRQ => {
                self.spsr_irq = self.cpsr;      
                self.banked_irq_regs.copy_from_slice(&self.gpr[SP..PC]);     
            },
            OperationModes::Supervisor => {
                self.spsr_svc = self.cpsr;
                self.banked_svc_regs.copy_from_slice(&self.gpr[SP..PC]);     
            },
            OperationModes::Abort => {
                self.spsr_abt = self.cpsr;
                self.banked_abt_regs.copy_from_slice(&self.gpr[SP..PC]);     
            },
            OperationModes::Undefined => {
                self.spsr_und = self.cpsr;
                self.banked_und_regs.copy_from_slice(&self.gpr[SP..PC]);     
            },
            // OperationModes::System => {} 
            _ => {
                self.spsr_user_sys = self.cpsr;
                self.banked_user_sys_regs.copy_from_slice(&self.gpr);     
            }
        }

        self.gpr[LR] = self.gpr[PC]; // Copy prev. mode PC reg into LR reg for new mode (generic regs)

        match new_mode {
            // OperationModes::User => {},
            OperationModes::FIQ => {
                // self.banked_fiq_regs[LR - 8] = self.gpr[PC];
                self.gpr[8..PC].copy_from_slice(&self.banked_fiq_regs);
            },
            OperationModes::IRQ => {
                // self.banked_irq_regs[1] = self.gpr[PC];
                self.gpr[SP..PC].copy_from_slice(&self.banked_irq_regs);
            },
            OperationModes::Supervisor => {
                // self.banked_svc_regs[1] = self.gpr[PC];
                self.gpr[SP..PC].copy_from_slice(&self.banked_svc_regs);
            },
            OperationModes::Abort => {
                // self.banked_abt_regs[1] = self.gpr[PC];
                self.gpr[SP..PC].copy_from_slice(&self.banked_abt_regs);
            },
            OperationModes::Undefined => {
                // self.banked_und_regs[1] = self.gpr[PC];
                self.gpr[SP..PC].copy_from_slice(&self.banked_und_regs);
            },
            // OperationModes::System => {}
            _ => {
                // self.banked_user_sys_regs[LR] = self.gpr[PC];
                self.gpr.copy_from_slice(&self.banked_user_sys_regs);
            }
        }
    }

    fn arise_exception(&mut self, exception: ExceptionType) {
        match exception {
            ExceptionType::Reset => {
                self.enter_operation_mode(OperationModes::Supervisor);
                self.set_cpsr_bit(CPSRBitsMask::F);
            },
            ExceptionType::UndefinedInstruction => {
                self.enter_operation_mode(OperationModes::Undefined);
            },
            ExceptionType::SoftwareInterrupt => {
                self.enter_operation_mode(OperationModes::Supervisor);
            },
            ExceptionType::PrefetchAbort => {
                self.enter_operation_mode(OperationModes::Abort);
            },
            ExceptionType::DataAbort => {
                self.enter_operation_mode(OperationModes::Abort);
            },
            ExceptionType::AddressExceeds => {
                self.enter_operation_mode(OperationModes::Supervisor);
            },
            ExceptionType::NormalInterrupt => {
                self.enter_operation_mode(OperationModes::IRQ);
            },
            ExceptionType::FastInterrupt => {
                self.enter_operation_mode(OperationModes::FIQ);
                self.set_cpsr_bit(CPSRBitsMask::F);
            }
        }

        self.set_cpsr_bit(CPSRBitsMask::T);
        self.cpu_mode = CpuStateMode::ARM;

        self.set_cpsr_bit(CPSRBitsMask::I);

        self.gpr[PC] = EXCEPTIONS_HANDLERS_ADDRESSES[exception as usize];
    }
}

// impl MemoryOperation for ARM7TDMI {
//     fn read8(&self, address: usize) -> u8 {
        
//     }

//     fn write8(&mut self, address: usize, value: u8) {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_increments_work_and_wraps() {
        let mut cpu = ARM7TDMI::new();
        let mut expected_pc = 4;

        cpu.increment_pc();

        assert_eq!(cpu.pc(), expected_pc);

        expected_pc = 3;
        cpu.pc_mut(0xFFFFFFFF);
        cpu.increment_pc();

        assert_eq!(cpu.pc(), expected_pc);
    }
}