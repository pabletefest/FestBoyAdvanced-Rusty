const SP: u8 = 13;
const LP: u8 = 14;
const PC: u8 = 15;

enum CpuStateMode
{
    ARM = 0,
    THUMB = 1
}

enum OperationModes {
    User = 16,
    FIQ = 17,
    IRQ = 18,
    Supervisor = 19,
    Abort = 23,
    Undefined = 27,
    System = 31
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

    instructionCycles: u32
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
            instructionCycles: 0u32
        }
    }
}