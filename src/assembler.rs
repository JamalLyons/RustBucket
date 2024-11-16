//! # VM Assembler
//! This module implements an assembler for converting assembly language into bytecode
//! for our virtual machine. The assembler performs a two-pass assembly process:
//! 1. First pass: Collect labels and calculate addresses
//! 2. Second pass: Generate actual bytecode

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Represents errors that can occur during the assembly process.
/// These errors provide detailed information about what went wrong during
/// assembly of the source code.
#[derive(Debug)]
pub enum AssemblerError
{
    /// Indicates an invalid or unknown instruction was encountered
    InvalidInstruction(String),
    /// Indicates an invalid register reference (must be r0-r7)
    InvalidRegister(String),
    /// Indicates an invalid value (e.g., number too large for u8)
    InvalidValue(String),
    /// Indicates a malformed label
    InvalidLabel(String),
    /// Indicates a reference to a non-existent label
    UndefinedLabel(String),
    /// Provides detailed information about operand count mismatches
    InvalidNumberOfOperands
    {
        instruction: String,
        expected: usize,
        got: usize,
    },
}

// Add Display implementation
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

// Add Error implementation
impl Error for AssemblerError {}

/// Represents a single instruction in the assembly code.
/// This struct holds all the information needed to generate
/// the bytecode for one instruction.
#[derive(Debug)]
struct Instruction
{
    /// The operation code (e.g., "MOV", "ADD", etc.)
    opcode: String,
    /// The operands for this instruction
    operands: Vec<String>,
}

/// The main assembler struct that handles converting assembly code to bytecode.
pub struct Assembler
{
    /// Vector of all instructions found in the source code
    instructions: Vec<Instruction>,
    /// HashMap of label names to their addresses in memory
    pub labels: HashMap<String, usize>,
    /// Tracks the current address during assembly
    current_address: usize,
}

impl Assembler
{
    /// Creates a new instance of the Assembler.
    ///
    /// # Returns
    /// * A new Assembler instance with empty instructions and labels.
    pub fn new() -> Self
    {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            current_address: 0,
        }
    }

    /// Performs the first pass of assembly, collecting labels and calculating addresses.
    ///
    /// # Arguments
    /// * `code` - The assembly source code as a string
    ///
    /// # Returns
    /// * `Result<(), AssemblerError>` - Ok if successful, Err with details if an error occurred
    fn first_pass(&mut self, code: &str) -> Result<(), AssemblerError>
    {
        self.current_address = 0;

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // Handle labels with validation
            if line.ends_with(':') {
                let label = line[..line.len() - 1].trim();
                self.validate_label(label)?;
                self.labels.insert(label.to_string(), self.current_address);
                continue;
            }

            // Parse instruction
            let parts: Vec<&str> = line.split(';').next().unwrap().trim().split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let opcode = parts[0].to_uppercase();
            let operands: Vec<String> = parts[1..].iter().map(|s| s.replace(',', "")).collect();

            let inst = Instruction { opcode, operands };

            self.update_instruction_address(&inst);
            self.instructions.push(inst);
        }
        Ok(())
    }

    /// Converts a register string (e.g., "r0", "r1") into its numeric value.
    ///
    /// # Arguments
    /// * `reg` - The register string to parse
    ///
    /// # Returns
    /// * `Result<u8, AssemblerError>` - The register number or an error if invalid
    fn parse_register(&self, reg: &str) -> Result<u8, AssemblerError>
    {
        if !reg.starts_with('r') {
            return Err(AssemblerError::InvalidRegister(reg.to_string()));
        }
        let num = u8::from_str(&reg[1..]).map_err(|_| AssemblerError::InvalidRegister(reg.to_string()))?;
        if num >= 8 {
            return Err(AssemblerError::InvalidRegister(reg.to_string()));
        }
        Ok(num)
    }

    /// Calculates how many bytes an instruction will occupy in memory.
    ///
    /// # Arguments
    /// * `inst` - Reference to the instruction to analyze
    ///
    /// # Returns
    /// * `usize` - The number of bytes this instruction will occupy
    fn calculate_instruction_size(&self, inst: &Instruction) -> usize
    {
        match inst.opcode.as_str() {
            "MOV" => 3,                                  // opcode + register + value
            "ADD" | "SUB" | "MUL" | "DIV" | "CMP" => 3,  // opcode + two registers
            "INC" | "DEC" | "OUT" | "PUSH" | "POP" => 2, // opcode + register
            "JMP" | "JEQ" | "JGT" => 2,                  // opcode + address
            "CALL" => 2,                                 // opcode + address
            "RET" | "HALT" => 1,                         // just opcode
            _ => 1,
        }
    }

    /// Converts an instruction into its bytecode representation.
    ///
    /// # Arguments
    /// * `inst` - Reference to the instruction to convert
    ///
    /// # Returns
    /// * `Result<Vec<u8>, AssemblerError>` - The bytecode or an error if conversion fails
    fn instruction_to_bytes(&self, inst: &Instruction) -> Result<Vec<u8>, AssemblerError>
    {
        // First validate the number of operands
        self.validate_operands(inst)?;

        let mut bytes = Vec::new();

        match inst.opcode.as_str() {
            // Register operations
            "MOV" => {
                bytes.push(0x04);
                bytes.push(self.parse_register(&inst.operands[0])?);
                bytes.push(self.parse_value(&inst.operands[1])?);
            }

            // Arithmetic operations
            "ADD" => self.encode_two_reg(0x30, &inst.operands, &mut bytes)?,
            "SUB" => self.encode_two_reg(0x31, &inst.operands, &mut bytes)?,
            "MUL" => self.encode_two_reg(0x32, &inst.operands, &mut bytes)?,
            "DIV" => self.encode_two_reg(0x33, &inst.operands, &mut bytes)?,

            // Single register operations
            "INC" => self.encode_single_reg(0x01, &inst.operands, &mut bytes)?,
            "DEC" => self.encode_single_reg(0x02, &inst.operands, &mut bytes)?,
            "OUT" => self.encode_single_reg(0x03, &inst.operands, &mut bytes)?,

            // Stack operations
            "PUSH" => self.encode_single_reg(0x10, &inst.operands, &mut bytes)?,
            "POP" => self.encode_single_reg(0x11, &inst.operands, &mut bytes)?,

            // Memory operations
            "LOAD" => {
                bytes.push(0x20);
                bytes.push(self.parse_register(&inst.operands[0])?);
                bytes.push(self.parse_memory_operand(&inst.operands[1])?);
            }
            "STORE" => {
                bytes.push(0x21);
                bytes.push(self.parse_register(&inst.operands[0])?);
                bytes.push(self.parse_memory_operand(&inst.operands[1])?);
            }
            "LDIDX" => self.encode_single_reg(0x22, &inst.operands, &mut bytes)?,
            "STIDX" => self.encode_single_reg(0x23, &inst.operands, &mut bytes)?,

            // Control flow
            "JMP" => self.encode_jump(0x40, &inst.operands, &mut bytes)?,
            "JEQ" => self.encode_jump(0x41, &inst.operands, &mut bytes)?,
            "JGT" => self.encode_jump(0x42, &inst.operands, &mut bytes)?,
            "CMP" => self.encode_two_reg(0x43, &inst.operands, &mut bytes)?,

            // Function calls
            "CALL" => self.encode_jump(0x12, &inst.operands, &mut bytes)?,
            "RET" => bytes.push(0x13),

            // System
            "HALT" => bytes.push(0xFF),

            _ => return Err(AssemblerError::InvalidInstruction(inst.opcode.clone())),
        }

        Ok(bytes)
    }

    // Helper methods for encoding different instruction types
    fn encode_two_reg(&self, opcode: u8, operands: &[String], bytes: &mut Vec<u8>) -> Result<(), AssemblerError>
    {
        bytes.push(opcode);
        bytes.push(self.parse_register(&operands[0])?);
        bytes.push(self.parse_register(&operands[1])?);
        Ok(())
    }

    fn encode_single_reg(&self, opcode: u8, operands: &[String], bytes: &mut Vec<u8>) -> Result<(), AssemblerError>
    {
        bytes.push(opcode);
        bytes.push(self.parse_register(&operands[0])?);
        Ok(())
    }

    fn encode_jump(&self, opcode: u8, operands: &[String], bytes: &mut Vec<u8>) -> Result<(), AssemblerError>
    {
        bytes.push(opcode);
        bytes.push(self.parse_jump_target(&operands[0])?);
        Ok(())
    }

    fn parse_value(&self, value: &str) -> Result<u8, AssemblerError>
    {
        // Handle both decimal and hexadecimal values
        let value = if value.starts_with("0x") {
            u8::from_str_radix(&value[2..], 16)
        } else {
            u8::from_str(value)
        }
        .map_err(|_| AssemblerError::InvalidValue(value.to_string()))?;
        Ok(value)
    }

    fn parse_jump_target(&self, target: &str) -> Result<u8, AssemblerError>
    {
        // First check if it's a label
        if let Some(&addr) = self.labels.get(target) {
            return Ok(addr as u8);
        }

        // If not a label, check if it's a hex value
        if target.starts_with("0x") {
            return u8::from_str_radix(&target[2..], 16).map_err(|_| AssemblerError::InvalidValue(target.to_string()));
        }

        // If it's not a valid number, it's an undefined label
        u8::from_str(target).map_err(|_| AssemblerError::UndefinedLabel(target.to_string()))
    }

    /// The main entry point for assembly. Converts assembly code into bytecode.
    ///
    /// # Arguments
    /// * `code` - The assembly source code to convert
    ///
    /// # Returns
    /// * `Result<Vec<u8>, AssemblerError>` - The generated bytecode or an error if assembly fails
    ///
    /// # Example
    /// ```
    /// let mut assembler = Assembler::new();
    /// let code = "
    ///     MOV r0, 5
    ///     MOV r1, 3
    ///     ADD r0, r1
    ///     HALT
    /// ";
    /// let bytecode = assembler.assemble(code)?;
    /// ```
    pub fn assemble(&mut self, code: &str) -> Result<Vec<u8>, AssemblerError>
    {
        self.first_pass(code)?;
        self.second_pass()
    }

    fn validate_operands(&self, inst: &Instruction) -> Result<(), AssemblerError>
    {
        let expected = match inst.opcode.as_str() {
            "MOV" | "ADD" | "SUB" | "MUL" | "DIV" | "CMP" => 2,
            "LOAD" | "STORE" => 2,
            "INC" | "DEC" | "OUT" | "PUSH" | "POP" | "LDIDX" | "STIDX" | "JMP" | "JEQ" | "JGT" => 1,
            "HALT" | "RET" => 0,
            _ => return Err(AssemblerError::InvalidInstruction(inst.opcode.clone())),
        };

        match inst.operands.len().cmp(&expected) {
            std::cmp::Ordering::Less => {
                return Err(AssemblerError::InvalidNumberOfOperands {
                    instruction: inst.opcode.clone(),
                    expected,
                    got: inst.operands.len(),
                })
            }
            std::cmp::Ordering::Greater => {
                return Err(AssemblerError::InvalidNumberOfOperands {
                    instruction: inst.opcode.clone(),
                    expected,
                    got: inst.operands.len(),
                })
            }
            std::cmp::Ordering::Equal => Ok(()),
        }
    }

    // Add this new method to handle memory addressing
    fn parse_memory_operand(&self, operand: &str) -> Result<u8, AssemblerError>
    {
        if operand.is_empty() {
            return Err(AssemblerError::InvalidValue("Empty operand".to_string()));
        }

        // If it starts with 'r', it's a register containing the address
        if operand.starts_with('r') {
            return self.parse_register(operand);
        }
        // Otherwise, treat it as a direct address
        self.parse_value(operand)
    }

    // Add this method to track instruction addresses during first pass
    fn update_instruction_address(&mut self, inst: &Instruction)
    {
        self.current_address += self.calculate_instruction_size(inst);
    }

    /// Performs the second pass of assembly, generating the final bytecode.
    /// This pass converts each instruction into its binary representation.
    ///
    /// # Returns
    /// * `Result<Vec<u8>, AssemblerError>` - The generated bytecode or an error if conversion fails
    fn second_pass(&self) -> Result<Vec<u8>, AssemblerError>
    {
        let mut bytecode = Vec::new();

        // Convert each instruction to bytes
        for inst in &self.instructions {
            let inst_bytes = self.instruction_to_bytes(inst)?;
            bytecode.extend(inst_bytes);
        }

        Ok(bytecode)
    }

    // Add label validation in first_pass
    fn validate_label(&self, label: &str) -> Result<(), AssemblerError>
    {
        // Labels should be alphanumeric and not start with a number
        if label.is_empty() || label.chars().next().unwrap().is_numeric() {
            return Err(AssemblerError::InvalidLabel(label.to_string()));
        }
        if !label.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(AssemblerError::InvalidLabel(label.to_string()));
        }
        Ok(())
    }
}
