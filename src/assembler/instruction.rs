//! Instruction representation for the assembler
//!
//! This module defines how assembly instructions are represented internally
//! during the assembly process, before they are converted to bytecode.

/// Represents a single assembly instruction.
///
/// Each instruction consists of an operation code (opcode) and zero or more
/// operands. For example, in the instruction "MOV r0, 5":
/// - "MOV" is the opcode
/// - "r0" and "5" are the operands
///
/// # Example
/// ```
/// let inst = Instruction {
///     opcode: "MOV".to_string(),
///     operands: vec!["r0".to_string(), "5".to_string()],
/// };
/// ```
#[derive(Debug)]
pub struct Instruction
{
    /// The operation code (e.g., "MOV", "ADD", "JMP")
    pub opcode: String,

    /// The instruction's operands (e.g., register names, values)
    /// For example:
    /// - "MOV r0, 5" has operands ["r0", "5"]
    /// - "ADD r0, r1" has operands ["r0", "r1"]
    /// - "HALT" has no operands (empty vector)
    pub operands: Vec<String>,
}

impl Instruction
{
    /// Creates a new instruction with the given opcode and operands.
    ///
    /// # Arguments
    /// * `opcode` - The operation code as a string
    /// * `operands` - A vector of operand strings
    pub fn new(opcode: String, operands: Vec<String>) -> Self
    {
        Self { opcode, operands }
    }
}
