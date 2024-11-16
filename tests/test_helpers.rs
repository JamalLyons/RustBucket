use vm::{VMConfig, CPU};

pub fn create_test_vm() -> CPU
{
    CPU::new(VMConfig::default())
}

pub fn assert_registers(vm: &CPU, expected: &[u8])
{
    for (i, &value) in expected.iter().enumerate() {
        assert_eq!(vm.get_register(i).unwrap(), value, "Register r{} has incorrect value", i);
    }
}
