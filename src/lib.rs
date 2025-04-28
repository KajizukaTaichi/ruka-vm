mod asm;
mod eval;
mod value;

pub use asm::asm;
pub use value::Value;

pub const MEMORY_SIZE: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct RukaVM {
    memory: [Value; MEMORY_SIZE],
    program: Vec<Instruction>,
    call: Vec<Value>,
    stack: Vec<Value>,
    pc: Value,
    ar: Value,
    dr: Value,
    cr: Value,
    ba: Value,
    sp: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Mov(Register, Operand),
    Add(Register, Operand),
    Mul(Register, Operand),
    Neg(Register),
    Inv(Register),
    Eql(Register, Operand),
    Les(Register, Operand),
    Nor(Register, Operand),
    Jmp(Operand, Operand),
    Cal(Operand),
    Ret,
    Lda(Register, Operand),
    Sta(Operand, Operand),
    Psh(Operand),
    Pop(Register),
    Nop,
    Hlt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    Literal(Value),
    Register(Register),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Pc,
    Ar,
    Dr,
    Cr,
    Ba,
    Sp,
}
