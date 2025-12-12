use vm_rs::vm::{Machine};

pub fn main() -> Result<(), &'static str> {
    let mut vm = Machine::new();

    let _ = vm.step();
    let _ = vm.step();
    let _ = vm.step();

    Ok(())
}