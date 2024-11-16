use std::error::Error;
use std::fmt;

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
            VMError::InvalidMemoryAccess(addr) => write!(f, "Invalid memory access at address {}", addr),
            VMError::InvalidRegister(reg) => write!(f, "Invalid register number {}", reg),
            VMError::InvalidOpcode(op) => write!(f, "Invalid opcode: {:#04x}", op),
        }
    }
}

impl Error for VMError {}
