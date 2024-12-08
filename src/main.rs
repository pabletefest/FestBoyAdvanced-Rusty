pub mod arm7tdmi;

fn main() {
    println!("Hello, world!");

    let _cpu = arm7tdmi::ARM7TDMI::new();
}
