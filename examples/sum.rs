use ruka_vm::*;

fn main() {
    let mut vm = RukaVM::new(vec![
        // スタックにデータを積む
        Instruction::Psh(Operand::Literal(10.0)), // 0
        Instruction::Psh(Operand::Literal(3.0)),  // 1
        Instruction::Psh(Operand::Literal(5.0)),  // 2
        // 関数呼び出し
        Instruction::Cal(Operand::Literal(5.0)), // 3
        Instruction::Hlt,                        // 4
        // スタックにある値を全部足す関数
        Instruction::Pop(Register::Rdx), // 5
        Instruction::Add(Register::Rax, Operand::Register(Register::Rdx)), // 6
        // ループのトップへ
        Instruction::Jmp(Operand::Register(Register::Rsp), Operand::Literal(5.0)), // 7
        Instruction::Ret,                                                          // 8
    ]);
    vm.run().unwrap();
    dbg!(&vm);
}
