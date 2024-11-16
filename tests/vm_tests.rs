use vm::assembler::Assembler;
use vm::{VMConfig, CPU};

#[test]
fn test_memory_operations()
{
    let test_cases = vec![
        (
            // Test LOAD and STORE
            r#"
            MOV r0, 42
            STORE r0, 0x50
            MOV r0, 0
            LOAD r1, 0x50
            HALT
            "#,
            vec![0, 42, 0, 0],
        ),
        (
            // Test indexed load/store
            r#"
            MOV r0, 42
            MOV r1, 0x50
            STIDX r0, r1
            MOV r0, 0
            LDIDX r2, r1
            HALT
            "#,
            vec![0, 0x50, 42, 0],
        ),
    ];

    run_test_cases(test_cases);
}

fn run_test_cases(test_cases: Vec<(&str, Vec<u8>)>)
{
    let mut assembler = Assembler::new();

    for (program, expected_registers) in test_cases {
        let mut vm = CPU::new(VMConfig::default());

        let bytecode = assembler
            .assemble(program)
            .unwrap_or_else(|_| panic!("Assembly failed for program:\n{}", program));
        vm.load_program(&bytecode);
        vm.run().expect("Program execution failed");

        // Check the first 4 registers against expected values
        for i in 0..4 {
            assert_eq!(
                vm.get_register(i).expect("Failed to get register"),
                expected_registers[i],
                "Register r{} has incorrect value",
                i
            );
        }
    }
}
