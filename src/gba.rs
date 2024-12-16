use crate::arm7tdmi::ARM7TDMI;
use crate::system_memory::SysMem;

const CYCLES_PER_FRAME: u32 = 280_896;

pub struct GBA {
    sys_mem: SysMem,
    cpu: ARM7TDMI
}

impl GBA {
    pub fn new() -> GBA {
        GBA {
            sys_mem: SysMem::new(),
            cpu: ARM7TDMI::new()
        }
    }

    pub fn run_frame(&mut self) {
        let total_cycles: u32 = 0;

        while total_cycles < CYCLES_PER_FRAME {
            let instruction_executed_cycles: u8 = self.cpu.run_instruction(&mut self.sys_mem);
        }
    }
}