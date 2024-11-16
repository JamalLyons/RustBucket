//! Error types for the assembler module
//!
//! This module defines all the possible errors that can occur during
//! the assembly process, from syntax errors to invalid instructions.

use std::error::Error;
use std::fmt;

/// Represents all possible errors that can occur during assembly.
#[derive(Debug)]
pub enum AssemblerError
{
    /// An invalid instruction opcode was encountered
    InvalidInstruction(String),

    /// A register reference was invalid (e.g., "r9" when only r0-r7 exist)
    InvalidRegister(String),

    /// An immediate value or address was invalid
    InvalidValue(String),

    /// A label was malformed or invalid
    InvalidLabel(String),

    /// A referenced label wasn't defined anywhere in the code
    UndefinedLabel(String),

    /// Wrong number of operands for an instruction
    InvalidNumberOfOperands
    {
        /// The instruction that had the wrong number of operands
        instruction: String,
        /// How many operands the instruction expects
        expected: usize,
        /// How many operands were actually provided
        got: usize,
    },

    /// An invalid address was encountered
    InvalidAddress(String),

    /// A syntax error occurred
    SyntaxError(String),
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
            AssemblerError::InvalidAddress(s) => write!(f, "Invalid address: {}", s),
            AssemblerError::SyntaxError(s) => write!(f, "Syntax error: {}", s),
        }
    }
}

impl Error for AssemblerError {}
