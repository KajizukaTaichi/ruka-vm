use crate::*;
use indexmap::IndexMap;

type Labels = IndexMap<String, f64>;

pub fn asm(source: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut labels = Labels::new();
    let mut index = 0;
    for line in source.lines() {
        let (line, _comment) = line.split_once(';').unwrap_or((line, ""));
        if let Some(label) = line.trim().strip_suffix(":") {
            labels.insert(label.trim().to_string(), index as f64);
        } else if !line.trim().is_empty() {
            index += 1;
        }
    }
    for line in source.lines() {
        let (line, _comment) = line.split_once(';').unwrap_or((line, ""));
        if let Some(mnemonic) = Instruction::asm(line.trim(), &mut labels) {
            instructions.push(mnemonic);
        }
    }

    index = 0;
    instructions.clear();
    for line in source.lines() {
        let (line, _comment) = line.split_once(';').unwrap_or((line, ""));
        if let Some(label) = line.trim().strip_suffix(":") {
            labels.insert(label.trim().to_string(), index as f64);
        } else if Instruction::asm(line.trim(), &mut labels).is_some() {
            index += 1;
        }
    }
    for line in source.lines() {
        let (line, _comment) = line.split_once(';').unwrap_or((line, ""));
        if let Some(mnemonic) = Instruction::asm(line.trim(), &mut labels) {
            instructions.push(mnemonic);
        }
    }
    Some(instructions)
}

impl Instruction {
    fn asm(source: &str, labels: &mut Labels) -> Option<Self> {
        let (opecode, operands) = source.split_once(' ').unwrap_or((source, ""));
        let operands = operands.split(',').collect::<Vec<_>>();
        Some(match opecode {
            "mov" => Self::Mov(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "add" => Self::Add(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "mul" => Self::Mul(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "neg" => Self::Neg(Register::asm(operands.get(0)?.trim())?),
            "inv" => Self::Inv(Register::asm(operands.get(0)?.trim())?),
            "eql" => Self::Eql(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "les" => Self::Les(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "nor" => Self::Nor(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "jmp" => Self::Jmp(
                Operand::asm(operands.get(0)?.trim(), labels)?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "cal" => Self::Cal(Operand::asm(operands.get(0)?.trim(), labels)?),
            "ret" => Self::Ret,
            "lda" => Self::Lda(
                Register::asm(operands.get(0)?.trim())?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "sta" => Self::Sta(
                Operand::asm(operands.get(0)?.trim(), labels)?,
                Operand::asm(operands.get(1)?.trim(), labels)?,
            ),
            "psh" => Self::Psh(Operand::asm(operands.get(0)?.trim(), labels)?),
            "pop" => Self::Pop(Register::asm(operands.get(0)?.trim())?),
            "nop" => Self::Nop,
            "hlt" => Self::Hlt,
            _ => return None,
        })
    }
}

impl Operand {
    fn asm(source: &str, labels: &mut Labels) -> Option<Self> {
        Some(if let Some(register) = Register::asm(source) {
            Self::Register(register)
        } else if let Ok(literal) = source.parse() {
            Self::Literal(literal)
        } else if let Some(literal) = labels.get(source) {
            Self::Literal(*literal)
        } else {
            return None;
        })
    }
}

impl Register {
    fn asm(source: &str) -> Option<Self> {
        Some(match source {
            "pc" => Self::Pc,
            "ar" => Self::Ar,
            "dr" => Self::Dr,
            "cr" => Self::Cr,
            "ba" => Self::Ba,
            "sp" => Self::Sp,
            _ => return None,
        })
    }
}
