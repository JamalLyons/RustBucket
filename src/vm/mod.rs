//! Virtual Machine implementation module
//!
//! This module contains the core components of our virtual machine:
//! - CPU: The processor that executes instructions
//! - Memory: Storage for program code and data
//! - Registers: Fast storage for values being processed
//!
//! The VM is configured using the VMConfig struct, which allows customization
//! of memory size, number of registers, and other important parameters.

pub mod cpu;
pub mod error;
pub mod opcode;

pub use cpu::CPU;
pub use error::VMError;
pub use opcode::Opcode;

// Default configuration values
const DEFAULT_MEMORY_SIZE: usize = 256; // Total memory in bytes
const DEFAULT_STACK_SIZE: usize = 64; // Size of the stack region
const DEFAULT_NUM_REGISTERS: usize = 8; // Number of general-purpose registers
const DEFAULT_PC_START: usize = 0; // Program Counter starts at memory address 0
const DEFAULT_SP_START: usize = 255; // Stack Pointer starts at top of memory

/// Configuration for the Virtual Machine
///
/// This struct allows customization of the VM's key parameters:
/// - Memory size and layout
/// - Number of registers
/// - Debug options
/// - Stack configuration
#[derive(Debug)]
pub struct VMConfig
{
    /// Total size of memory in bytes
    pub memory_size: usize,
    /// Enable debug output for instruction execution
    pub debug: bool,
    /// Size of the stack region in bytes
    pub stack_size: usize,
    /// Number of general-purpose registers available
    pub num_registers: usize,
    /// Starting address for the Program Counter
    pub pc_start: usize,
    /// Starting address for the Stack Pointer
    pub sp_start: usize,
}

impl Default for VMConfig
{
    /// Creates a default configuration with reasonable values for a simple VM
    fn default() -> Self
    {
        Self {
            memory_size: DEFAULT_MEMORY_SIZE,
            debug: false,
            stack_size: DEFAULT_STACK_SIZE,
            num_registers: DEFAULT_NUM_REGISTERS,
            pc_start: DEFAULT_PC_START,
            sp_start: DEFAULT_SP_START,
        }
    }
}

impl VMConfig
{
    /// Creates a new VMConfig with custom memory size and debug setting
    /// Other values are set to defaults
    ///
    /// # Arguments
    /// * `memory_size` - Total memory size in bytes
    /// * `debug` - Enable debug output
    ///
    /// # Example
    /// ```
    /// let config = VMConfig::new(512, true); // 512 bytes of memory, debug enabled
    /// ```
    pub fn new(memory_size: usize, debug: bool) -> Self
    {
        Self {
            memory_size,
            debug,
            ..Default::default()
        }
    }

    /// Creates a new VMConfig builder for fluent configuration
    ///
    /// # Example
    /// ```
    /// let config = VMConfig::builder()
    ///     .memory_size(512)
    ///     .debug(true)
    ///     .stack_size(128)
    ///     .build();
    /// ```
    pub fn builder() -> VMConfigBuilder
    {
        VMConfigBuilder::default()
    }
}

/// Builder for creating customized VM configurations
///
/// This struct provides a fluent interface for setting up
/// a VMConfig with custom values. Each method returns self
/// to allow method chaining.
pub struct VMConfigBuilder
{
    config: VMConfig,
}

impl Default for VMConfigBuilder
{
    fn default() -> Self
    {
        Self {
            config: VMConfig::default(),
        }
    }
}

impl VMConfigBuilder
{
    /// Set the total memory size in bytes
    pub fn memory_size(mut self, size: usize) -> Self
    {
        self.config.memory_size = size;
        self
    }

    /// Enable or disable debug output
    pub fn debug(mut self, debug: bool) -> Self
    {
        self.config.debug = debug;
        self
    }

    /// Set the stack size in bytes
    pub fn stack_size(mut self, size: usize) -> Self
    {
        self.config.stack_size = size;
        self
    }

    /// Set the number of general-purpose registers
    pub fn num_registers(mut self, num: usize) -> Self
    {
        self.config.num_registers = num;
        self
    }

    /// Set the starting address for the Program Counter
    pub fn pc_start(mut self, addr: usize) -> Self
    {
        self.config.pc_start = addr;
        self
    }

    /// Set the starting address for the Stack Pointer
    pub fn sp_start(mut self, addr: usize) -> Self
    {
        self.config.sp_start = addr;
        self
    }

    /// Build the final VMConfig with all settings applied
    pub fn build(self) -> VMConfig
    {
        // TODO: Add validation of configuration values
        self.config
    }
}
