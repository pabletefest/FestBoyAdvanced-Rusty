use crate::system_memory::{MemoryOperation, SysMem};

const SP: usize = 13;
const LP: usize = 14;
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

pub struct ARM7TDMI {
    gpr: [u32; 16],
    banked_fiq_regs: [u32; 7],
    banked_svc_regs: [u32; 2],
    banked_abt_regs: [u32; 2],
    banked_irq_regs: [u32; 2],
    banked_und_regs: [u32; 2],
    cpsr: u32,
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
            banked_fiq_regs: [0; 7],
            banked_svc_regs: [0; 2],
            banked_abt_regs: [0; 2],
            banked_irq_regs: [0; 2],
            banked_und_regs: [0; 2],
            cpsr: 0u32,
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
    
    fn reset(&mut self) {

    }

    fn fetch_opcode(&self, sys_mem: &mut SysMem) -> u32 {
        0
    }

    fn decode_and_execute_opcode(&mut self) {

    }

    // fn execute_opcode(&mut self) {

    // }

    pub fn run_instruction(&mut self, sys_mem: &mut SysMem) -> u8 {
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

    fn enter_operation_mode(&mut self, mode: OperationModes) {    
        self.cpsr = (self.cpsr & 0x0000001F) | mode as u32;
        self.operation_mode = mode;
        
        match mode {
            OperationModes::User => {},
            OperationModes::FIQ => {
                self.banked_fiq_regs[LP - 8] = self.gpr[PC];
                self.spsr_fiq = self.cpsr;
            },
            OperationModes::IRQ => {
                self.banked_irq_regs[1] = self.gpr[PC];
                self.spsr_irq = self.cpsr;           
            },
            OperationModes::Supervisor => {
                self.banked_svc_regs[1] = self.gpr[PC];
                self.spsr_svc = self.cpsr;
            },
            OperationModes::Abort => {
                self.banked_abt_regs[1] = self.gpr[PC];
                self.spsr_abt = self.cpsr;
            },
            OperationModes::Undefined => {
                self.banked_und_regs[1] = self.gpr[PC];
                self.spsr_und = self.cpsr;
            },
            OperationModes::System => {} 
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