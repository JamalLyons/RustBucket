/// Represents a single instruction in the assembly code.
/// This struct holds all the information needed to generate
/// the bytecode for one instruction.
#[derive(Debug)]
pub struct Instruction
{
    /// The operation code (e.g., "MOV", "ADD", etc.)
    pub opcode: String,
    /// The operands for this instruction
    pub operands: Vec<String>,
}
