/// Enum representing the different opcodes the VM can execute.
/// Each opcode represents a specific instruction that the VM can process.
#[derive(Debug)]
enum Opcode
{
    // Register Operations
    /// Increment: Adds 1 to the value in the specified register
    /// Usage: Inc(reg)
    /// Example: Inc(0) increments register 0
    Inc(u8),

    /// Decrement: Subtracts 1 from the value in the specified register
    /// Usage: Dec(reg)
    /// Example: Dec(0) decrements register 0
    Dec(u8),

    /// Output: Prints the value from the specified register to stdout
    /// Usage: Out(reg)
    /// Example: Out(0) prints the value in register 0
    Out(u8),

    /// Move: Loads an immediate value into the specified register
    /// Usage: Mov(dst_reg, immediate_value)
    /// Example: Mov(0, 5) loads the value 5 into register 0
    Mov(u8, u8),

    // Stack Operations
    /// Push: Pushes the value from the specified register onto the stack
    /// Usage: Push(reg)
    /// Example: Push(0) pushes the value from register 0 onto the stack
    Push(u8),

    /// Pop: Pops a value from the stack into the specified register
    /// Usage: Pop(reg)
    /// Example: Pop(0) pops the top stack value into register 0
    Pop(u8),

    /// Call: Pushes the next instruction address onto the call stack and jumps to the specified address
    /// Usage: Call followed by address byte
    /// Example: [0x12, 0x20] calls subroutine at address 0x20
    Call,

    /// Return: Pops the top address from the call stack and jumps to it
    /// Usage: Ret
    /// Example: 0x13 returns from subroutine
    Ret,

    // Memory Operations
    /// Load: Loads a value from memory into the specified register
    /// Usage: Load(reg) followed by address byte
    /// Example: [0x20, 0x00, 0x50] loads value at address 0x50 into register 0
    Load(u8),

    /// Store: Stores the value from the specified register into memory
    /// Usage: Store(reg) followed by address byte
    /// Example: [0x21, 0x00, 0x50] stores value from register 0 to address 0x50
    Store(u8),

    /// Load Indexed: Loads a value from memory using base address plus index
    /// Usage: LdIdx(reg) followed by base address byte
    /// The index is taken from register 1
    /// Example: [0x22, 0x00, 0x50] loads value from (0x50 + r1) into register 0
    LdIdx(u8),

    /// Store Indexed: Stores a value to memory using base address plus index
    /// Usage: StIdx(reg) followed by base address byte
    /// The index is taken from register 1
    /// Example: [0x23, 0x00, 0x50] stores value from register 0 to (0x50 + r1)
    StIdx(u8),

    // Arithmetic Operations
    /// Add: Adds the value from src register to dst register
    /// Usage: Add(dst_reg, src_reg)
    /// Example: Add(0, 1) adds r1 to r0, storing result in r0
    Add(u8, u8),

    /// Subtract: Subtracts the value in src register from dst register
    /// Usage: Sub(dst_reg, src_reg)
    /// Example: Sub(0, 1) subtracts r1 from r0, storing result in r0
    Sub(u8, u8),

    /// Multiply: Multiplies dst register by src register
    /// Usage: Mul(dst_reg, src_reg)
    /// Example: Mul(0, 1) multiplies r0 by r1, storing result in r0
    Mul(u8, u8),

    /// Divide: Divides dst register by src register
    /// Usage: Div(dst_reg, src_reg)
    /// Example: Div(0, 1) divides r0 by r1, storing result in r0
    /// Note: Triggers DivisionByZero error if src register contains 0
    Div(u8, u8),

    // Control Flow Operations
    /// Jump: Unconditional jump to specified address
    /// Usage: Jmp followed by address byte
    /// Example: [0x40, 0x20] jumps to address 0x20
    Jmp,

    /// Jump if Equal: Jumps if the zero flag is set (last comparison was equal)
    /// Usage: Jeq followed by address byte
    /// Example: [0x41, 0x20] jumps to 0x20 if zero flag is set
    Jeq,

    /// Jump if Greater: Jumps if the greater flag is set (last comparison was greater)
    /// Usage: Jgt followed by address byte
    /// Example: [0x42, 0x20] jumps to 0x20 if greater flag is set
    Jgt,

    /// Compare: Compares two registers and sets flags
    /// Usage: Cmp(reg1, reg2)
    /// Sets zero flag if reg1 == reg2
    /// Sets greater flag if reg1 > reg2
    /// Example: Cmp(0, 1) compares r0 with r1
    Cmp(u8, u8),

    /// Halt: Stops the program execution
    /// Usage: Halt (0xFF)
    /// Example: 0xFF halts the program
    Halt,

    /// Unknown: Represents an invalid or unsupported opcode
    /// Contains the invalid opcode byte that was encountered
    Unknown(u8),
}

impl From<u8> for Opcode
{
    /// Converts a raw byte (u8) into an `Opcode` enum variant.
    ///
    /// # Arguments
    /// * `byte` - The raw opcode byte fetched from memory.
    ///
    /// # Returns
    /// * `Opcode` - The corresponding enum variant for the opcode.
    fn from(byte: u8) -> Self
    {
        match byte {
            0x01 => {
                // Instructions with register operands need to read next byte
                Opcode::Inc(0) // Placeholder - actual register will be read in fetch
            }
            0x02 => Opcode::Dec(0),
            0x03 => Opcode::Out(0),
            0x04 => Opcode::Mov(0, 0),
            0x10 => Opcode::Push(0),
            0x11 => Opcode::Pop(0),
            0x12 => Opcode::Call,
            0x13 => Opcode::Ret,
            0x20 => Opcode::Load(0),
            0x21 => Opcode::Store(0),
            0x22 => Opcode::LdIdx(0),
            0x23 => Opcode::StIdx(0),
            0x30 => Opcode::Add(0, 0),
            0x31 => Opcode::Sub(0, 0),
            0x32 => Opcode::Mul(0, 0),
            0x33 => Opcode::Div(0, 0),
            0x40 => Opcode::Jmp,
            0x41 => Opcode::Jeq,
            0x42 => Opcode::Jgt,
            0x43 => Opcode::Cmp(0, 0),
            0xFF => Opcode::Halt,
            _ => Opcode::Unknown(byte),
        }
    }
}

/// A struct representing a simple CPU for the virtual machine.
struct CPU
{
    registers: [u8; 8], // A
    pc: usize,          // Program counter to keep track of the next instruction.
    memory: Vec<u8>,    // Simulated RAM to store program instructions and data.
    stack: Vec<u8>,     // Stack to store temporary data
    sp: usize,          // Stack pointer
    // Add flags register for comparisons
    flags: u8,
    // Add call stack for function calls
    call_stack: Vec<usize>,

    debug: bool,
}

impl CPU
{
    /// Creates a new CPU with a specified amount of memory.
    ///
    /// # Arguments
    /// * `memory_size` - The size of the memory (RAM) to allocate for the CPU.
    fn new(memory_size: usize, debug: bool) -> Self
    {
        Self {
            registers: [0; 8],            // Initialize all registers to 0.
            pc: 0,                        // Start the program counter at the beginning of memory.
            memory: vec![0; memory_size], // Allocate memory with the given size, initialized to 0.
            stack: vec![0; 256],          // Initialize stack with 256 bytes
            sp: 0,
            flags: 0,
            call_stack: Vec::new(),
            debug,
        }
    }

    /// Loads a program (an array of instructions) into the VM's memory.
    ///
    /// # Arguments
    /// * `program` - A slice of bytes representing the program's instructions.
    fn load_program(&mut self, program: &[u8])
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
            0x40..=0x42 => match opcode_byte {
                0x40 => Opcode::Jmp,
                0x41 => Opcode::Jeq,
                0x42 => Opcode::Jgt,
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
        if self.debug {
            println!("Executing opcode: {:?}", opcode);
            self.dump_state();
        }

        match opcode {
            Opcode::Inc(reg) => {
                if reg >= 8 {
                    return Err(VMError::InvalidRegister(reg));
                }
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(1);
            }
            Opcode::Dec(reg) => {
                if reg >= 8 {
                    return Err(VMError::InvalidRegister(reg));
                }
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_sub(1);
            }
            Opcode::Out(reg) => print!("{}", self.registers[reg as usize]),
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
                if self.sp >= self.stack.len() {
                    return Err(VMError::StackOverflow);
                }
                self.stack[self.sp] = self.registers[reg as usize];
                self.sp += 1;
            }

            Opcode::Pop(reg) => {
                if self.sp == 0 {
                    return Err(VMError::StackUnderflow);
                }
                self.sp -= 1;
                self.registers[reg as usize] = self.stack[self.sp];
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
                // Set program counter to end of memory to stop execution
                self.pc = self.memory.len();
            }
        }
        Ok(())
    }

    /// Runs the program loaded into the VM's memory.
    ///
    /// This function repeatedly fetches, decodes, and executes instructions until
    /// the program halts or the memory is exhausted.
    fn run(&mut self) -> Result<(), VMError>
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
    fn dump_state(&self)
    {
        if !self.debug {
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
    }
}

use std::fmt;

/// Custom error type for VM operations
#[derive(Debug)]
pub enum VMError
{
    StackOverflow,
    StackUnderflow,
    DivisionByZero,
    InvalidMemoryAccess(usize),
    InvalidRegister(u8),
    InvalidOpcode(u8),
}

impl fmt::Display for VMError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            VMError::StackOverflow => write!(f, "Stack overflow"),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::InvalidMemoryAccess(addr) => {
                write!(f, "Invalid memory access at address {}", addr)
            }
            VMError::InvalidRegister(reg) => write!(f, "Invalid register number {}", reg),
            VMError::InvalidOpcode(op) => write!(f, "Invalid opcode: {:#04x}", op),
        }
    }
}

fn main()
{
    let mut vm = CPU::new(256, false); // Set debug to true to see what's happening

    // Program to add 5 + 3
    let add_numbers = [
        // Load first number (5) into register 0
        0x04, 0x00, 0x05, // MOV r0, 5     ; r0 = 5
        // Load second number (3) into register 1
        0x04, 0x01, 0x03, // MOV r1, 3     ; r1 = 3
        // Add r1 to r0 (result will be in r0)
        0x30, 0x00, 0x01, // ADD r0, r1    ; r0 = r0 + r1
        // Output the result from register 0
        0x03, 0x00, // OUT r0        ; print result
        // Halt the program
        0xFF, // HALT
    ];

    println!("Running program to add 5 + 3");
    print!("Result: ");

    vm.load_program(&add_numbers);

    match vm.run() {
        Ok(_) => println!("\nProgram completed successfully"),
        Err(e) => eprintln!("\nProgram failed: {}", e),
    }
}
