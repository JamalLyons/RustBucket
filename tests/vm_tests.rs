use vm::assembler::Assembler;
use vm::{VMConfig, CPU};

#[test]
fn test_register_operations()
{
    let test_cases = vec![
        (
            // Test MOV immediate
            "
            MOV r0, 5
            MOV r1, 10
            HLT
            ",
            vec![0, 5, 1, 10],
        ),
        (
            // Test MOV register to register
            "
            MOV r0, 42
            MOV r1, r0
            HLT
            ",
            vec![42, 42, 0, 0],
        ),
        (
            // Test INC and DEC
            "
            MOV r0, 5
            INC r0
            MOV r1, 10
            DEC r1
            HLT
            ",
            vec![6, 9, 0, 0],
        ),
    ];

    run_test_cases(test_cases);
}

#[test]
fn test_arithmetic_operations()
{
    let test_cases = vec![
        (
            // Test ADD
            "
            MOV r0, 5
            MOV r1, 3
            ADD r0, r1
            HLT
            ",
            vec![8, 3, 0, 0],
        ),
        (
            // Test SUB
            "
            MOV r0, 10
            MOV r1, 3
            SUB r0, r1
            HLT
            ",
            vec![7, 3, 0, 0],
        ),
        (
            // Test MUL
            "
            MOV r0, 4
            MOV r1, 3
            MUL r0, r1
            HLT
            ",
            vec![12, 3, 0, 0],
        ),
        (
            // Test DIV
            "
            MOV r0, 15
            MOV r1, 3
            DIV r0, r1
            HLT
            ",
            vec![5, 3, 0, 0],
        ),
    ];

    run_test_cases(test_cases);
}

#[test]
fn test_memory_operations()
{
    let test_cases = vec![
        (
            // Test LOAD and STORE
            "
            MOV r0, 42
            STORE r0, 0x50
            MOV r0, 0
            LOAD r1, 0x50
            HLT
            ",
            vec![0, 42, 0, 0],
        ),
        (
            // Test indexed load/store
            "
            MOV r0, 42
            MOV r1, 0x50
            STXI r0, r1
            MOV r0, 0
            LDXI r2, r1
            HLT
            ",
            vec![0, 0x50, 42, 0],
        ),
    ];

    run_test_cases(test_cases);
}

#[test]
fn test_control_flow()
{
    let test_cases = vec![
        (
            // Test JMP
            "
            MOV r0, 1
            JMP skip
            MOV r0, 2
            skip:
            MOV r1, 3
            HLT
            ",
            vec![1, 3, 0, 0],
        ),
        (
            // Test conditional jumps
            "
            MOV r0, 5
            MOV r1, 3
            CMP r0, r1
            JGT greater
            MOV r2, 0
            greater:
            MOV r2, 1
            HLT
            ",
            vec![5, 3, 1, 0],
        ),
    ];

    run_test_cases(test_cases);
}

#[test]
fn test_stack_operations()
{
    let test_cases = vec![
        (
            // Test PUSH and POP
            "
            MOV r0, 42
            PUSH r0
            MOV r0, 0
            POP r1
            HLT
            ",
            vec![0, 42, 0, 0],
        ),
        (
            // Test CALL and RET
            "
            MOV r0, 1
            CALL subroutine
            MOV r2, 3
            HLT
            subroutine:
            MOV r1, 2
            RET
            ",
            vec![1, 2, 3, 0],
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
            .expect(&format!("Assembly failed for program:\n{}", program));
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

#[test]
#[should_panic(expected = "Division by zero")]
fn test_division_by_zero()
{
    let program = "
        MOV r0, 10
        MOV r1, 0
        DIV r0, r1
        HLT
    ";

    let mut assembler = Assembler::new();
    let mut vm = CPU::new(VMConfig::default());

    let bytecode = assembler.assemble(program).expect("Assembly failed");
    vm.load_program(&bytecode);
    vm.run().expect("Program should fail with division by zero");
}

#[test]
fn test_comparison_flags()
{
    let test_cases = vec![
        (
            // Test equal comparison
            "
            MOV r0, 5
            MOV r1, 5
            CMP r0, r1
            JEQ equal
            MOV r2, 0
            JMP end
            equal:
            MOV r2, 1
            end:
            HLT
            ",
            vec![5, 5, 1, 0],
        ),
        (
            // Test not equal comparison
            "
            MOV r0, 5
            MOV r1, 6
            CMP r0, r1
            JNE not_equal
            MOV r2, 0
            JMP end
            not_equal:
            MOV r2, 1
            end:
            HLT
            ",
            vec![5, 6, 1, 0],
        ),
    ];

    run_test_cases(test_cases);
}
