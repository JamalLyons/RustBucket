use std::fmt;

#[derive(Debug)]
pub enum VMError
{
    InvalidRegister(usize),
    InvalidMemoryAccess(usize),
    StackOverflow,
    StackUnderflow,
    DivisionByZero,
    InvalidOpcode(u8),
    ProgramComplete,
}

impl std::error::Error for VMError {}

impl fmt::Display for VMError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            VMError::InvalidRegister(reg) => write!(f, "Invalid register access: {}", reg),
            VMError::InvalidMemoryAccess(addr) => write!(f, "Invalid memory access at address: {}", addr),
            VMError::StackOverflow => write!(f, "Stack overflow"),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::InvalidOpcode(op) => write!(f, "Invalid opcode: {:#04x}", op),
            VMError::ProgramComplete => write!(f, "Program completed execution"),
        }
    }
}
