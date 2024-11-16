use super::error::VMError;
use super::{Opcode, VMConfig};

/// A struct representing a simple CPU for the virtual machine.
pub struct CPU
{
    registers: Vec<u8>,
    pc: usize,
    memory: Vec<u8>,
    sp: usize,
    flags: u8,
    config: VMConfig,
    call_stack: Vec<usize>,
}

impl CPU
{
    /// Creates a new CPU with a specified amount of memory.
    ///
    /// # Arguments
    /// * `config` - The configuration for the CPU.
    pub fn new(config: VMConfig) -> Self
    {
        // Calculate stack pointer start based on stack size
        let sp_start = config.memory_size - config.stack_size;

        Self {
            registers: vec![0; config.num_registers],
            pc: config.pc_start,
            memory: vec![0; config.memory_size],
            sp: sp_start, // Use calculated stack pointer
            flags: 0,
            config,
            call_stack: Vec::new(),
        }
    }

    /// Loads a program (an array of instructions) into the VM's memory.
    ///
    /// # Arguments
    /// * `program` - A slice of bytes representing the program's instructions.
    pub fn load_program(&mut self, program: &[u8])
    {
        // Copy the program's bytes into the VM's memory starting at address 0.
        self.memory[..program.len()].copy_from_slice(program);
    }

    /// Fetches the next instruction from memory and increments the program counter.
    ///
    /// # Returns
    /// * `Opcode` - The next opcode (instruction) to be executed.
    fn fetch(&mut self) -> Opcode
    {
        if self.pc >= self.memory.len() {
            return Opcode::Halt;
        }

        let opcode_byte = self.memory[self.pc];
        self.pc += 1;

        match opcode_byte {
            // Single register instructions
            0x01..=0x03 => {
                let reg = self.memory[self.pc];
                self.pc += 1;
                match opcode_byte {
                    0x01 => Opcode::Inc(reg),
                    0x02 => Opcode::Dec(reg),
                    0x03 => Opcode::Out(reg),
                    _ => unreachable!(),
                }
            }

            // Two register instructions
            0x04 | 0x30..=0x33 | 0x43 => {
                let dst = self.memory[self.pc];
                self.pc += 1;
                let src = self.memory[self.pc];
                self.pc += 1;
                match opcode_byte {
                    0x04 => Opcode::Mov(dst, src),
                    0x30 => Opcode::Add(dst, src),
                    0x31 => Opcode::Sub(dst, src),
                    0x32 => Opcode::Mul(dst, src),
                    0x33 => Opcode::Div(dst, src),
                    0x43 => Opcode::Cmp(dst, src),
                    _ => unreachable!(),
                }
            }

            // Register and memory address instructions
            0x20..=0x23 => {
                let reg = self.memory[self.pc];
                self.pc += 1;
                match opcode_byte {
                    0x20 => Opcode::Load(reg),
                    0x21 => Opcode::Store(reg),
                    0x22 => Opcode::LdIdx(reg),
                    0x23 => Opcode::StIdx(reg),
                    _ => unreachable!(),
                }
            }

            // Jump instructions
            0x40..=0x44 => match opcode_byte {
                0x40 => Opcode::Jmp,
                0x41 => Opcode::Jeq,
                0x42 => Opcode::Jgt,
                0x44 => Opcode::Jne,
                _ => unreachable!(),
            },

            // Stack operations
            0x10..=0x11 => {
                let reg = self.memory[self.pc];
                self.pc += 1;
                match opcode_byte {
                    0x10 => Opcode::Push(reg),
                    0x11 => Opcode::Pop(reg),
                    _ => unreachable!(),
                }
            }

            // Function calls
            0x12 => Opcode::Call,
            0x13 => Opcode::Ret,

            0xFF => Opcode::Halt,
            _ => Opcode::Unknown(opcode_byte),
        }
    }

    /// Executes a single instruction based on the given opcode.
    ///
    /// # Arguments
    /// * `opcode` - The instruction to execute.
    fn execute(&mut self, opcode: Opcode) -> Result<(), VMError>
    {
        match opcode {
            Opcode::Inc(reg) => {
                let reg = reg as usize;
                if reg >= self.config.num_registers {
                    return Err(VMError::InvalidRegister(reg));
                }
                self.registers[reg] = self.registers[reg].wrapping_add(1);
            }
            Opcode::Dec(reg) => {
                let reg = reg as usize;
                if reg >= self.config.num_registers {
                    return Err(VMError::InvalidRegister(reg));
                }
                self.registers[reg] = self.registers[reg].wrapping_sub(1);
            }
            Opcode::Out(reg) => {
                print!("{} ", self.registers[reg as usize]);
                if self.config.debug {
                    println!();
                }
            }
            Opcode::Mov(dst, src) => {
                self.registers[dst as usize] = src;
            }

            Opcode::Add(dst, src) => {
                self.registers[dst as usize] = self.registers[dst as usize].wrapping_add(self.registers[src as usize]);
            }
            Opcode::Sub(dst, src) => {
                self.registers[dst as usize] = self.registers[dst as usize].wrapping_sub(self.registers[src as usize]);
            }
            Opcode::Mul(dst, src) => {
                self.registers[dst as usize] = self.registers[dst as usize].wrapping_mul(self.registers[src as usize]);
            }
            Opcode::Div(dst, src) => {
                if self.registers[src as usize] == 0 {
                    return Err(VMError::DivisionByZero);
                }
                self.registers[dst as usize] = self.registers[dst as usize].wrapping_div(self.registers[src as usize]);
            }

            Opcode::Cmp(reg1, reg2) => {
                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                self.set_zero_flag(val1 == val2);
                self.set_greater_flag(val1 > val2);
            }

            Opcode::Jmp => {
                let addr = self.memory[self.pc] as usize;
                self.pc = addr;
            }
            Opcode::Jeq => {
                let addr = self.memory[self.pc] as usize;
                self.pc += 1;
                if self.flags & 1 != 0 {
                    self.pc = addr;
                }
            }
            Opcode::Jgt => {
                let addr = self.memory[self.pc] as usize;
                self.pc += 1;
                if self.flags & 2 != 0 {
                    self.pc = addr;
                }
            }

            Opcode::Call => {
                let addr = self.memory[self.pc] as usize;
                self.pc += 1;
                self.call_stack.push(self.pc);
                self.pc = addr;
            }
            Opcode::Ret => {
                if let Some(return_addr) = self.call_stack.pop() {
                    self.pc = return_addr;
                }
            }

            Opcode::Push(reg) => {
                if self.sp == 0 {
                    return Err(VMError::StackOverflow);
                }
                self.sp -= 1;
                self.memory[self.sp] = self.registers[reg as usize];
            }

            Opcode::Pop(reg) => {
                if self.sp >= self.memory.len() {
                    return Err(VMError::StackUnderflow);
                }
                self.registers[reg as usize] = self.memory[self.sp];
                self.sp += 1;
            }

            Opcode::Load(reg) => {
                let addr = self.memory[self.pc] as usize;
                self.pc += 1;
                if addr >= self.memory.len() {
                    return Err(VMError::InvalidMemoryAccess(addr));
                }
                self.registers[reg as usize] = self.memory[addr];
            }

            Opcode::Store(reg) => {
                let addr = self.memory[self.pc] as usize;
                self.pc += 1;
                if addr >= self.memory.len() {
                    return Err(VMError::InvalidMemoryAccess(addr));
                }
                self.memory[addr] = self.registers[reg as usize];
            }

            Opcode::LdIdx(reg) => {
                let base_addr = self.memory[self.pc] as usize;
                self.pc += 1;
                let index = self.registers[1] as usize;
                let addr = base_addr + index;
                if addr >= self.memory.len() {
                    return Err(VMError::InvalidMemoryAccess(addr));
                }
                self.registers[reg as usize] = self.memory[addr];
            }

            Opcode::StIdx(reg) => {
                let base_addr = self.memory[self.pc] as usize;
                self.pc += 1;
                let index = self.registers[1] as usize;
                let addr = base_addr + index;
                if addr >= self.memory.len() {
                    return Err(VMError::InvalidMemoryAccess(addr));
                }
                self.memory[addr] = self.registers[reg as usize];
            }

            Opcode::Unknown(byte) => {
                return Err(VMError::InvalidOpcode(byte));
            }

            Opcode::Halt => {
                self.pc = self.memory.len();
                return Ok(());
            }

            Opcode::Jne => {
                let addr = self.memory[self.pc] as usize;
                self.pc += 1;
                if self.flags & 1 == 0 {
                    // Jump if zero flag is NOT set
                    self.pc = addr;
                }
            }
        }
        Ok(())
    }

    /// Runs the program loaded into the VM's memory.
    ///
    /// This function repeatedly fetches, decodes, and executes instructions until
    /// the program halts or the memory is exhausted.
    pub fn run(&mut self) -> Result<(), VMError>
    {
        while self.pc < self.memory.len() {
            let opcode = self.fetch();
            self.execute(opcode)?;
        }
        Ok(())
    }

    // Add helper methods for flag operations
    fn set_zero_flag(&mut self, value: bool)
    {
        if value {
            self.flags |= 1;
        } else {
            self.flags &= !1;
        }
    }

    fn set_greater_flag(&mut self, value: bool)
    {
        if value {
            self.flags |= 2;
        } else {
            self.flags &= !2;
        }
    }

    // Add this method
    pub fn dump_state(&self)
    {
        if !self.config.debug {
            return;
        }

        println!("\nCPU State:");
        println!("PC: {}", self.pc);
        println!("Registers: {:?}", self.registers);
        println!("Flags: {:08b}", self.flags);
        println!("Stack pointer: {}", self.sp);
        println!(
            "Memory[0x50-0x52]: {:02X} {:02X} {:02X}",
            self.memory[0x50], self.memory[0x51], self.memory[0x52]
        );
        println!("Call stack: {:?}", self.call_stack);
    }

    // Add this new method
    pub fn get_register(&self, index: usize) -> Result<u8, VMError>
    {
        if index >= self.registers.len() {
            return Err(VMError::InvalidRegister(index));
        }
        Ok(self.registers[index])
    }
}
