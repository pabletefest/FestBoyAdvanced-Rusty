pub mod gba;
pub mod system_memory;
pub mod arm7tdmi;
pub mod arm_instructions;
pub mod thumb_instructions;

fn main() {
    println!("Hello, world!");

    // let _cpu: arm7tdmi::ARM7TDMI = arm7tdmi::ARM7TDMI::new();
    let gba: gba::GBA = gba::GBA::new();
}