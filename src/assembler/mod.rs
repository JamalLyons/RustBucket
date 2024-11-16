//! This module implements an assembler for converting assembly language into bytecode
//! for our virtual machine. The assembler performs a two-pass assembly process:
//! 1. First pass: Collect labels and calculate addresses
//! 2. Second pass: Generate actual bytecode

mod error;
mod instruction;
mod parser;

use parser::Parser;

/// The main assembler struct that handles converting assembly code to bytecode.
pub struct Assembler
{
    parser: Parser,
}

impl Assembler
{
    /// Creates a new instance of the Assembler.
    pub fn new() -> Self
    {
        Self { parser: Parser::new() }
    }

    /// The main entry point for assembly. Converts assembly code into bytecode.
    pub fn assemble(&mut self, code: &str) -> Result<Vec<u8>, error::AssemblerError>
    {
        self.parser.assemble(code)
    }

    /// Get access to the labels map for debugging
    pub fn labels(&self) -> &std::collections::HashMap<String, usize>
    {
        &self.parser.labels
    }
}
