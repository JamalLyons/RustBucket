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
- `INC reg` : Increment register
- `DEC reg` : Decrement register
- `MOV reg, val` : Load immediate value into register
- `MOV reg, reg` : Copy value from one register to another

#### Arithmetic Operations
- `ADD dst, src` : Add src register to dst register
- `SUB dst, src` : Subtract src register from dst register
- `MUL dst, src` : Multiply dst register by src register
- `DIV dst, src` : Divide dst register by src register

#### Memory Operations
- `LOAD reg, addr` : Load from memory address into register
- `STORE reg, addr` : Store register into memory address
- `LDXI reg, reg` : Load indexed (address in second register)
- `STXI reg, reg` : Store indexed (address in second register)

#### Control Flow
- `JMP addr` : Unconditional jump
- `JEQ addr` : Jump if equal
- `JNE addr` : Jump if not equal
- `JGT addr` : Jump if greater
- `JLT addr` : Jump if less
- `JLE addr` : Jump if less or equal
- `JGE addr` : Jump if greater or equal
- `CMP r1, r2` : Compare registers

#### Stack Operations
- `PUSH reg` : Push register onto stack
- `POP reg` : Pop from stack into register
- `CALL addr` : Call subroutine
- `RET` : Return from subroutine

#### System Operations
- `HLT` : Halt execution
- `NOP` : No operation
- `OUT reg` : Output register value

## Example Programs

### Adding Two Numbers 

```assembly
    MOV r0, 5    ; Load 5 into r0
    MOV r1, 3    ; Load 3 into r1
    ADD r0, r1   ; Add r1 to r0
    OUT r0       ; Output result
    HLT         ; Stop execution
```

### Using the Stack

```assembly
    MOV r0, 7    ; Load 7 into r0
    PUSH r0      ; Push r0 onto stack
    MOV r0, 3    ; Load 3 into r0
    POP r1       ; Pop into r1
    ADD r0, r1   ; Add r1 to r0
    OUT r0       ; Output result
    HLT         ; Stop execution
```

## Error Handling

The VM includes comprehensive error handling for:
- Stack overflow/underflow
- Division by zero
- Invalid memory access
- Invalid register numbers
- Unknown opcodes
- Invalid instruction formats

## Data Types

The VM currently supports a limited set of data types focused on basic 8-bit operations:

### Primary Data Types
- **8-bit Unsigned Integers (u8)**
  - All register values
  - Memory values
  - Stack values
  - Immediate instruction values
  - All arithmetic operations use wrapping arithmetic

### Internal Types
- **Addresses/Indices (usize)**
  - Program counter (PC)
  - Stack pointer (SP)
  - Call stack addresses

### Flags
- **Status Flags (bits)**
  - Zero flag (bit 0): Set when a comparison results in equality
  - Greater flag (bit 1): Set when a comparison results in greater than

### Limitations
The VM currently does not support:
- Signed integers
- Multi-byte integers
- Floating point numbers
- Characters/strings
- Complex data structures

## Usage

```rust
let mut vm = CPU::new(VMConfig::default());  // 256 bytes of memory
let mut assembler = Assembler::new();

 match assembler.assemble(assembly_code) {
  Ok(bytecode) => {
    vm.load_program(&bytecode);
    vm.run();
  }
  Err(e) => {
    eprintln!("Assembly failed: {}", e);
  }
 }

```

## Features

- [x] Basic VM implementation
- [x] Simple assembler
- [x] Comprehensive instruction set
- [ ] Add support for more data types (e.g. 16-bit integers)
- [ ] Add support for complex data structures (e.g. arrays, maps)
- [ ] Add support for interrupts
- [ ] Add I/O operations
- [ ] Add debugging features