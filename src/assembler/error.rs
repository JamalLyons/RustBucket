use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AssemblerError
{
    InvalidInstruction(String),
    InvalidRegister(String),
    InvalidValue(String),
    InvalidLabel(String),
    UndefinedLabel(String),
    InvalidNumberOfOperands
    {
        instruction: String,
        expected: usize,
        got: usize,
    },
}

impl fmt::Display for AssemblerError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            AssemblerError::InvalidInstruction(s) => write!(f, "Invalid instruction: {}", s),
            AssemblerError::InvalidRegister(s) => write!(f, "Invalid register: {}", s),
            AssemblerError::InvalidValue(s) => write!(f, "Invalid value: {}", s),
            AssemblerError::InvalidLabel(s) => write!(f, "Invalid label: {}", s),
            AssemblerError::UndefinedLabel(s) => write!(f, "Undefined label: {}", s),
            AssemblerError::InvalidNumberOfOperands {
                instruction,
                expected,
                got,
            } => write!(
                f,
                "Invalid number of operands for {}: expected {}, got {}",
                instruction, expected, got
            ),
        }
    }
}

impl Error for AssemblerError {}
