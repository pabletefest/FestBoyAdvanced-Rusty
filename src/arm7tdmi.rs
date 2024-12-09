const SP: u8 = 13;
const LP: u8 = 14;
const PC: u8 = 15;

const MODE_BITS_MASK: u32 = 0x0000001F;

const EXCEPTIONS_HANDLERS_ADDRESSES: [u32; 8] = [0x00000000, 0x00000004, 0x00000008, 0x0000000C, 0x00000010, 0x00000014, 0x00000018, 0x0000001C];

#[derive(Clone, Copy)]
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

    pipeline: [u32; 2],

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
            pipeline: [0; 2],
            instruction_cycles: 0u32
        }
    }

    fn reset(&mut self) {

    }

    fn run_instruction(&mut self) -> u8 {
        0
    }

    fn set_cpsr_bit(&mut self, bit_mask: CPSRBitsMask) {
        self.cpsr |= bit_mask as u32;
    }

    fn clear_cpsr_bit(&mut self, bit_mask: CPSRBitsMask) {
        self.cpsr &= !(bit_mask as u32);
    }

    fn enter_operation_mode(&mut self, mode: OperationModes) {
        
        self.cpsr = (self.cpsr & 0x0000001F) | mode as u32;
        
        match mode {
            OperationModes::User => {},
            OperationModes::FIQ => {},
            OperationModes::IRQ => {},
            OperationModes::Supervisor => {},
            OperationModes::Abort => {},
            OperationModes::Undefined => {},
            OperationModes::System => {} 
        }
    }
}