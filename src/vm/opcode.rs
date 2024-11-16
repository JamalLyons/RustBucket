/// Enum representing the different opcodes the VM can execute.
/// Each opcode represents a specific instruction that the VM can process.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode
{
    // Single register instructions
    Inc(u8),
    Dec(u8),
    Out(u8),

    // Two register/value instructions
    Mov(u8, u8),
    Add(u8, u8),
    Sub(u8, u8),
    Mul(u8, u8),
    Div(u8, u8),
    Cmp(u8, u8),

    // Memory operations
    Load(u8),
    Store(u8),
    LdIdx(u8),
    StIdx(u8),

    // Stack operations
    Push(u8),
    Pop(u8),

    // Control flow
    Call,
    Ret,
    Jmp,
    Jeq,
    Jgt,

    // System
    Halt,
    Unknown(u8),
}

impl From<u8> for Opcode
{
    fn from(byte: u8) -> Self
    {
        match byte {
            0x01 => Opcode::Inc(0),
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
