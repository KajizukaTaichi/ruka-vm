use ruka_vm::*;

fn main() {
    let bytecode = asm(include_str!("sum.asm").trim()).unwrap();
    dbg!(&bytecode);
    let mut vm = RukaVM::new(bytecode);
    vm.run().unwrap();
    vm.dump();
}
