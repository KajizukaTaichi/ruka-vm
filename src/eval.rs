use crate::*;
use colored::*;
use std::mem::size_of;

impl RukaVM {
    pub fn new(program: Vec<Instruction>) -> Self {
        println!("Welcome to Ruka VM!");
        println!("{} Bytes free", size_of::<[f64; MEMORY_SIZE]>());

        RukaVM {
            program,
            call: Vec::new(),
            stack: Vec::new(),
            memory: [Value::new(0.0); MEMORY_SIZE],
            pc: Value::new(0.0),
            ar: Value::new(0.0),
            dr: Value::new(0.0),
            cr: Value::new(0.0),
            ba: Value::new(0.0),
            sp: Value::new(0.0),
        }
    }

    pub fn run(&mut self) -> Option<()> {
        loop {
            let instruction = self.program.get(self.pc.as_f64() as usize)?.clone();
            match instruction {
                Instruction::Mov(reg, val) => {
                    let val = self.get_operand(val);
                    let reg = self.get_register(reg);
                    *reg = val
                }
                Instruction::Add(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    reg.num = reg.den * operand.den + operand.num * reg.den;
                    reg.den = reg.den * operand.den;
                }
                Instruction::Mul(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    reg.den = reg.den * operand.num;
                    reg.num = reg.num * operand.den;
                }
                Instruction::Neg(reg) => {
                    let reg = self.get_register(reg);
                    reg.sign = !reg.sign
                }
                Instruction::Inv(reg) => {
                    let reg = self.get_register(reg);
                    reg.num = reg.den;
                    reg.den = reg.num;
                }
                Instruction::Eql(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = Value::new(if *reg == operand { 1.0 } else { 0.0 })
                }
                Instruction::Les(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    let result = reg.as_f64() < operand.as_f64();
                    *reg = Value::new(if result { 1.0 } else { 0.0 })
                }
                Instruction::Nor(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    let result = !(reg.num != 0 || operand.num != 0);
                    *reg = Value::new(if result { 1.0 } else { 0.0 })
                }
                Instruction::Jmp(cond, addr) => {
                    let cond = self.get_operand(cond);
                    let addr = self.get_operand(addr);
                    if cond.num != 0 {
                        self.pc = addr;
                        continue;
                    }
                }
                Instruction::Cal(addr) => {
                    let addr = self.get_operand(addr);
                    self.call.push(self.pc);
                    self.pc = addr;
                    continue;
                }
                Instruction::Ret => self.pc = self.call.pop()?,
                Instruction::Lda(reg, addr) => {
                    let addr = self.get_operand(addr);
                    let val = self.memory.get(addr.as_f64() as usize)?.clone();
                    let reg = self.get_register(reg);
                    *reg = val
                }
                Instruction::Sta(addr, val) => {
                    let addr = self.get_operand(addr);
                    let val = self.get_operand(val);
                    let addr = self.memory.get_mut(addr.as_f64() as usize)?;
                    *addr = val;
                }
                Instruction::Psh(val) => {
                    let val = self.get_operand(val);
                    self.stack.push(val);
                    self.sp = Value::new(self.stack.len() as f64);
                }
                Instruction::Pop(reg) => {
                    let val = self.stack.pop()?;
                    let reg = self.get_register(reg);
                    *reg = val;
                    self.sp = Value::new(self.stack.len() as f64);
                }
                Instruction::Nop => {}
                Instruction::Hlt => break,
            }
            self.pc = Value::new(self.pc.as_f64() + 1.0);
        }
        Some(())
    }

    pub fn dump(&self) {
        macro_rules! view {
            ($val: expr) => {{
                let formatted = format!("{:08}", $val);
                if $val.as_f64() != 0.0 {
                    formatted.bold()
                } else {
                    formatted.normal()
                }
            }};
        }

        println!("Registers:");
        println!(" PC: {:08}  AR: {:08}", view!(self.pc), view!(self.ar));
        println!(" DR: {:08}  CR: {:08}", view!(self.dr), view!(self.cr));
        println!(" BA: {:08}  SP: {:08}", view!(self.ba), view!(self.sp));

        println!("Stack Area:");
        for (i, val) in self.stack.iter().enumerate() {
            println!(" {}: {}", i, view!(*val));
        }

        println!("Memory Area:");
        for (i, vals) in self.memory.chunks(8).enumerate() {
            let i = i * 8;
            print!(" {i:02} ~ {:02}: ", i + 7);
            for val in vals {
                print!("{} ", view!(*val));
            }
            println!()
        }
    }

    fn get_register(&mut self, register: Register) -> &mut Value {
        match register {
            Register::Pc => &mut self.pc,
            Register::Ar => &mut self.ar,
            Register::Dr => &mut self.dr,
            Register::Cr => &mut self.cr,
            Register::Ba => &mut self.ba,
            Register::Sp => &mut self.sp,
        }
    }

    fn get_operand(&mut self, operand: Operand) -> Value {
        match operand {
            Operand::Literal(value) => value,
            Operand::Register(register) => self.get_register(register).clone(),
        }
    }
}
