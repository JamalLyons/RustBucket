//! Instruction representation for the assembler
//!
//! This module defines how assembly instructions are represented internally
//! during the assembly process, before they are converted to bytecode.

use std::str::FromStr;

use super::error::AssemblerError;

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
#[derive(Debug, PartialEq)]
pub struct Instruction
{
    pub opcode: String,
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

    /// Encode the instruction to a byte array.
    pub fn encode(&self) -> Result<Vec<u8>, AssemblerError>
    {
        match self.opcode.as_str() {
            "MOV" => {
                check_operand_count(self, 2)?;
                let dst = parse_register(&self.operands[0])?;
                let src = if self.operands[1].starts_with('r') {
                    parse_register(&self.operands[1])?
                } else {
                    parse_value(&self.operands[1])?
                };
                Ok(vec![0x04, dst, src])
            }
            "STORE" => {
                check_operand_count(self, 2)?;
                let reg = parse_register(&self.operands[0])?;
                let addr = parse_value(&self.operands[1])?;
                Ok(vec![0x21, reg, addr])
            }
            "LOAD" => {
                check_operand_count(self, 2)?;
                let reg = parse_register(&self.operands[0])?;
                let addr = parse_value(&self.operands[1])?;
                Ok(vec![0x20, reg, addr])
            }
            "STIDX" => {
                check_operand_count(self, 2)?;
                let reg = parse_register(&self.operands[0])?;
                let base = parse_register(&self.operands[1])?;
                Ok(vec![0x23, reg, base])
            }
            "LDIDX" => {
                check_operand_count(self, 2)?;
                let reg = parse_register(&self.operands[0])?;
                let base = parse_register(&self.operands[1])?;
                Ok(vec![0x22, reg, base])
            }
            "PUSH" => {
                check_operand_count(self, 1)?;
                let reg = parse_register(&self.operands[0])?;
                Ok(vec![0x10, reg])
            }
            "POP" => {
                check_operand_count(self, 1)?;
                let reg = parse_register(&self.operands[0])?;
                Ok(vec![0x11, reg])
            }
            "CALL" => {
                check_operand_count(self, 1)?;
                let addr = parse_value(&self.operands[0])?;
                Ok(vec![0x12, addr])
            }
            "RET" => {
                check_operand_count(self, 0)?;
                Ok(vec![0x13])
            }
            "JMP" => {
                check_operand_count(self, 1)?;
                let addr = parse_value(&self.operands[0])?;
                Ok(vec![0x40, addr])
            }
            "JEQ" => {
                check_operand_count(self, 1)?;
                let addr = parse_value(&self.operands[0])?;
                Ok(vec![0x41, addr])
            }
            "JGT" => {
                check_operand_count(self, 1)?;
                let addr = parse_value(&self.operands[0])?;
                Ok(vec![0x42, addr])
            }
            "JNE" => {
                check_operand_count(self, 1)?;
                let addr = parse_value(&self.operands[0])?;
                Ok(vec![0x44, addr])
            }
            "CMP" => {
                check_operand_count(self, 2)?;
                let reg1 = parse_register(&self.operands[0])?;
                let reg2 = parse_register(&self.operands[1])?;
                Ok(vec![0x43, reg1, reg2])
            }
            "HALT" | "HLT" => {
                check_operand_count(self, 0)?;
                Ok(vec![0xFF])
            }
            _ => Err(AssemblerError::InvalidInstruction(self.opcode.clone())),
        }
    }
}

impl FromStr for Instruction
{
    type Err = AssemblerError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let s = s.trim();

        // Skip empty lines and comments
        if s.is_empty() || s.starts_with(';') {
            return Err(AssemblerError::SyntaxError("Empty or comment line".to_string()));
        }

        // Split into parts and handle comments
        let parts: Vec<&str> = s
            .split(';')  // Split on comments
            .next()      // Take the part before any comment
            .unwrap()    // We know there's at least one part
            .split_whitespace()
            .flat_map(|part| part.split(','))
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .collect();

        if parts.is_empty() {
            return Err(AssemblerError::SyntaxError("Empty instruction".to_string()));
        }

        let opcode = parts[0].to_uppercase();
        let operands = parts[1..].iter().map(|s| s.to_string()).collect();

        Ok(Instruction::new(opcode, operands))
    }
}

// Helper functions
fn parse_register(reg: &str) -> Result<u8, AssemblerError>
{
    if !reg.starts_with('r') {
        return Err(AssemblerError::InvalidRegister(reg.to_string()));
    }
    let num = reg[1..]
        .parse::<u8>()
        .map_err(|_| AssemblerError::InvalidRegister(reg.to_string()))?;
    if num >= 8 {
        return Err(AssemblerError::InvalidRegister(reg.to_string()));
    }
    Ok(num)
}

fn parse_value(val: &str) -> Result<u8, AssemblerError>
{
    if val.starts_with("0x") {
        u8::from_str_radix(&val[2..], 16)
    } else {
        val.parse()
    }
    .map_err(|_| AssemblerError::InvalidValue(val.to_string()))
}

fn check_operand_count(inst: &Instruction, expected: usize) -> Result<(), AssemblerError>
{
    if inst.operands.len() != expected {
        return Err(AssemblerError::InvalidNumberOfOperands {
            instruction: inst.opcode.clone(),
            expected,
            got: inst.operands.len(),
        });
    }
    Ok(())
}

fn encode_two_reg_op(inst: &Instruction, opcode: u8) -> Result<Vec<u8>, AssemblerError>
{
    check_operand_count(inst, 2)?;
    Ok(vec![
        opcode,
        parse_register(&inst.operands[0])?,
        parse_register(&inst.operands[1])?,
    ])
}
