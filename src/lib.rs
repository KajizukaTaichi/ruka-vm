mod asm;
mod eval;
pub use asm::asm;

pub const MEMORY_SIZE: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct RukaVM {
    memory: [f64; MEMORY_SIZE],
    program: Vec<Instruction>,
    call: Vec<f64>,
    stack: Vec<f64>,
    pc: f64,
    rax: f64,
    rdx: f64,
    rcx: f64,
    rbx: f64,
    rsp: f64,
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
    Sys(Operand),
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
    Literal(f64),
    Register(Register),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Pc,
    Rax,
    Rdx,
    Rcx,
    Rbx,
    Rsp,
}
