use ruka_vm::*;

fn main() {
    let mut vm = RukaVM::new(vec![
        // スタックにデータを積む
        Instruction::Psh(Operand::Literal(10.0)), // 0
        Instruction::Psh(Operand::Literal(3.0)),  // 1
        Instruction::Psh(Operand::Literal(5.0)),  // 2
        // 関数呼び出し
        Instruction::Cal(Operand::Literal(6.0)), // 3
        Instruction::Sta(Operand::Literal(0.0), Operand::Register(Register::Rax)), // 4
        Instruction::Hlt,                        // 5
        // スタックにある値を全部足す関数
        Instruction::Pop(Register::Rdx), // 6
        Instruction::Add(Register::Rax, Operand::Register(Register::Rdx)), // 7
        // スタックに値がある限り繰り返す
        Instruction::Jmp(Operand::Register(Register::Rsp), Operand::Literal(6.0)), // 8
        Instruction::Ret,                                                          // 9
    ]);
    vm.run();
    vm.dump();
}
