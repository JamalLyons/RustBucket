# Simple Virtual Machine Implementation in Rust

This project implements a simple virtual machine (VM) that simulates a basic computer architecture. It's designed for educational purposes to demonstrate how computers work at a low level.

## Architecture Overview

### CPU Components
- **Registers**: 8 general-purpose 8-bit registers (r0-r7)
- **Program Counter (PC)**: Points to the next instruction to execute
- **Flags Register**: Stores comparison results
  - Bit 0: Zero flag (set when comparison result is equal)
  - Bit 1: Greater flag (set when first value is greater)
- **Stack**: 256 bytes of stack memory
- **Memory**: Configurable size (default 256 bytes)

### Memory Layout
- **0x00 - 0x4F**: Program instructions
- **0x50 - 0xFF**: Data storage
- Stack grows from the end of memory downward

### Instruction Set

#### Register Operations
- `INC reg` (0x01): Increment register
- `DEC reg` (0x02): Decrement register
- `OUT reg` (0x03): Output register value
- `MOV reg, val` (0x04): Load immediate value into register

#### Arithmetic Operations
- `ADD dst, src` (0x30): Add src register to dst register
- `SUB dst, src` (0x31): Subtract src register from dst register
- `MUL dst, src` (0x32): Multiply dst register by src register
- `DIV dst, src` (0x33): Divide dst register by src register

#### Memory Operations
- `LOAD reg` (0x20): Load from memory into register
- `STORE reg` (0x21): Store register into memory
- `LDIDX reg` (0x22): Load indexed
- `STIDX reg` (0x23): Store indexed

#### Control Flow
- `JMP addr` (0x40): Unconditional jump
- `JEQ addr` (0x41): Jump if equal
- `JGT addr` (0x42): Jump if greater
- `CMP r1, r2` (0x43): Compare registers

#### Stack Operations
- `PUSH reg` (0x10): Push register onto stack
- `POP reg` (0x11): Pop from stack into register
- `CALL addr` (0x12): Call subroutine
- `RET` (0x13): Return from subroutine

## Example Programs

### Adding Two Numbers 

```rust
let program = [
0x04, 0x00, 0x05, // MOV r0, 5
0x04, 0x01, 0x03, // MOV r1, 3
0x30, 0x00, 0x01, // ADD r0, r1
0x03, 0x00, // OUT r0
0xFF, // HALT
];
```

### Using the Stack

```rust
let program = [
0x04, 0x00, 0x07, // MOV r0, 7
0x10, 0x00, // PUSH r0
0x04, 0x00, 0x03, // MOV r0, 3
0x11, 0x01, // POP r1
0x30, 0x00, 0x01, // ADD r0, r1
0x03, 0x00, // OUT r0
0xFF, // HALT
];
```

## Error Handling

The VM includes comprehensive error handling for:
- Stack overflow/underflow
- Division by zero
- Invalid memory access
- Invalid register numbers
- Unknown opcodes

## Usage

```rust
// Create a new VM instance
let mut vm = CPU::new(256, false);  // 256 bytes of memory, debugging disabled

// Load and run a program
vm.load_program(&program);

match vm.run() {
    Ok(_) => println!("Program completed successfully"),
    Err(e) => eprintln!("Program failed: {}", e),
}
```

## Todo

- [ ] Create a simple assembler to make writing programs easier
- [ ] Add more instructions
- [ ] Add support for more data types
- [ ] Add support for loops
- [ ] Add support for functions