pub mod arm7tdmi;
pub mod system_memory;

fn main() {
    println!("Hello, world!");

    let _cpu = arm7tdmi::ARM7TDMI::new();
}
