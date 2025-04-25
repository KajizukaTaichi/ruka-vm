#[derive(Debug, Clone, PartialEq)]
pub struct RukaVM {
    memory: [f64; 64],
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

impl RukaVM {
    pub fn new(program: Vec<Instruction>) -> Self {
        println!("Welcome to Ruka VM!");
        println!("512 bytes free");

        RukaVM {
            pc: 0.0,
            program,
            call: Vec::new(),
            stack: Vec::new(),
            memory: [0.0; 64],
            rax: 0.0,
            rbx: 0.0,
            rcx: 0.0,
            rdx: 0.0,
            rsp: 0.0,
        }
    }

    pub fn run(&mut self) -> Option<()> {
        loop {
            let instruction = self.program.get(self.pc as usize)?.clone();
            match instruction {
                Instruction::Mov(reg, val) => {
                    let val = self.get_operand(val);
                    let reg = self.get_register(reg);
                    *reg = val
                }
                Instruction::Add(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = *reg + operand
                }
                Instruction::Mul(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = *reg * operand
                }
                Instruction::Neg(reg) => {
                    let reg = self.get_register(reg);
                    *reg = -*reg
                }
                Instruction::Inv(reg) => {
                    let reg = self.get_register(reg);
                    *reg = 1.0 / *reg;
                }
                Instruction::Eql(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = if *reg == operand { 1.0 } else { 0.0 }
                }
                Instruction::Les(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    *reg = if *reg < operand { 1.0 } else { 0.0 }
                }
                Instruction::Nor(dst, src) => {
                    let operand = self.get_operand(src);
                    let reg = self.get_register(dst);
                    let result = !(*reg != 0.0 || operand != 0.0);
                    *reg = if result { 1.0 } else { 0.0 }
                }
                Instruction::Jmp(cond, addr) => {
                    let cond = self.get_operand(cond);
                    let addr = self.get_operand(addr);
                    if cond != 0.0 {
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
                    let val = self.memory.get(addr as usize)?.clone();
                    let reg = self.get_register(reg);
                    *reg = val
                }
                Instruction::Sta(addr, val) => {
                    let addr = self.get_operand(addr);
                    let val = self.get_operand(val);
                    let addr = self.memory.get_mut(addr as usize)?;
                    *addr = val;
                }
                Instruction::Psh(val) => {
                    let val = self.get_operand(val);
                    self.stack.push(val);
                    self.rsp += 1.0;
                }
                Instruction::Pop(reg) => {
                    let val = self.stack.pop()?;
                    let reg = self.get_register(reg);
                    *reg = val;
                    self.rsp -= 1.0;
                }
                Instruction::Nop => {}
                Instruction::Hlt => break,
            }
            self.pc += 1.0;
        }
        Some(())
    }

    pub fn dump(&self) {
        println!("Registers:");
        print!(" PC : {:08}\t", self.pc);
        println!("RAX: {:08}", self.rax);
        print!(" RDX: {:08}\t", self.rdx);
        println!("RCX: {:08}", self.rcx);
        print!(" RBX: {:08}\t", self.rbx);
        println!("RSP: {:08}", self.rsp);

        println!("Stack Area:");
        for (i, val) in self.stack.iter().enumerate() {
            println!(" {}: {}", i, val);
        }

        println!("Memory Area:");
        for (i, vals) in self.memory.chunks(8).enumerate() {
            let i = i * 8;
            print!(" {i:02} ~ {:02}: ", i + 7);
            for val in vals {
                print!("{:08} ", val);
            }
            println!()
        }
    }

    fn get_register(&mut self, register: Register) -> &mut f64 {
        match register {
            Register::Pc => &mut self.pc,
            Register::Rax => &mut self.rax,
            Register::Rdx => &mut self.rdx,
            Register::Rcx => &mut self.rcx,
            Register::Rbx => &mut self.rbx,
            Register::Rsp => &mut self.rsp,
        }
    }

    fn get_operand(&mut self, operand: Operand) -> f64 {
        match operand {
            Operand::Literal(value) => value,
            Operand::Register(register) => self.get_register(register).clone(),
        }
    }
}
