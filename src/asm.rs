use crate::*;

pub fn asm(source: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    for line in source.lines() {
        let (line, _comment) = line.split_once(';').unwrap_or((line, ""));
        if let Some(mnemonic) = Instruction::asm(line) {
            instructions.push(mnemonic);
        }
    }
    Some(instructions)
}

impl Instruction {
    fn asm(source: &str) -> Option<Self> {
        let (opecode, operands) = source.split_once(' ').unwrap_or((source, ""));
        let operands = operands.split(',').collect::<Vec<_>>();
        Some(match opecode {
            "mov" => Self::Mov(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "add" => Self::Add(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "mul" => Self::Mul(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "neg" => Self::Neg(Register::asm(operands.get(0)?.trim())?),
            "inv" => Self::Inv(Register::asm(operands.get(0)?.trim())?),
            "eql" => Self::Eql(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "les" => Self::Les(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "jmp" => Self::Jmp(
                Operand::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "cal" => Self::Cal(Operand::asm(operands.get(0)?.trim())?),
            "ret" => Self::Ret,
            "lda" => Self::Lda(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "sta" => Self::Sta(
                Operand::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim())?,
            ),
            "psh" => Self::Psh(Operand::asm(operands.get(0)?.trim())?),
            "pop" => Self::Pop(Register::asm(operands.get(0)?.trim())?),
            "nop" => Self::Nop,
            "hlt" => Self::Hlt,
            _ => return None,
        })
    }
}

impl Operand {
    fn asm(source: &str) -> Option<Self> {
        Some(if let Some(register) = Register::asm(source) {
            Self::Register(register)
        } else if let Ok(literal) = source.parse() {
            Self::Literal(literal)
        } else {
            return None;
        })
    }
}

impl Register {
    fn asm(source: &str) -> Option<Self> {
        Some(match source {
            "pc" => Self::Pc,
            "rax" => Self::Rax,
            "rdx" => Self::Rdx,
            "rcx" => Self::Rcx,
            "rbx" => Self::Rbx,
            "rsp" => Self::Rsp,
            _ => return None,
        })
    }
}
