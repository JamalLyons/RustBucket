pub mod cpu;
pub mod error;
pub mod opcode;

pub use opcode::Opcode;

const DEFAULT_MEMORY_SIZE: usize = 256;
const DEFAULT_STACK_SIZE: usize = 64;
const DEFAULT_NUM_REGISTERS: usize = 8;
const DEFAULT_PC_START: usize = 0;
const DEFAULT_SP_START: usize = 255;

/// Configuration for the VM
pub struct VMConfig
{
    /// Size of memory in bytes
    pub memory_size: usize,
    /// Enable debug output
    pub debug: bool,
    /// Size of the stack in bytes
    pub stack_size: usize,
    /// Number of general-purpose registers
    pub num_registers: usize,
    /// Program counter start address
    pub pc_start: usize,
    /// Stack pointer start address
    pub sp_start: usize,
}

impl Default for VMConfig
{
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
    /// Creates a new VMConfig with custom settings
    pub fn new(memory_size: usize, debug: bool) -> Self
    {
        Self {
            memory_size,
            debug,
            ..Default::default()
        }
    }

    /// Creates a new VMConfig builder with default settings
    pub fn builder() -> VMConfigBuilder
    {
        VMConfigBuilder::default()
    }
}

/// Builder for VMConfig to allow fluent configuration
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
    /// Set the memory size in bytes
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

    /// Set the program counter start address
    pub fn pc_start(mut self, addr: usize) -> Self
    {
        self.config.pc_start = addr;
        self
    }

    /// Set the stack pointer start address
    pub fn sp_start(mut self, addr: usize) -> Self
    {
        self.config.sp_start = addr;
        self
    }

    /// Build the final VMConfig
    pub fn build(self) -> VMConfig
    {
        self.config
    }
}
