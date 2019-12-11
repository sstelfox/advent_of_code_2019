use super::*;

type FaultResult = Result<(), Fault>;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_advancing() -> FaultResult {
    init_logger();

    let mut ic = IntCodeComputer::from_str("1,0,0,0,2,0,0,0,99")?;

    ic.advance(4)?;
    assert_eq!(ic.program_counter(), 4);
    ic.advance(2)?;
    assert_eq!(ic.program_counter(), 6);
    ic.advance(1)?;
    assert_eq!(ic.program_counter(), 7);

    let mut ic = IntCodeComputer {
        pc: MEMORY_SIZE - 1,

        input: Vec::new(),
        memory: [None; MEMORY_SIZE],
        output: Vec::new(),

        waiting_on_input: false,
        original_memory: [None; MEMORY_SIZE],
    };

    // Allow advancing to equal to the memory size (allow halt to be the final instruction)
    ic.advance(1)?;
    assert_eq!(ic.program_counter(), MEMORY_SIZE);

    // Ensure we can't advance any further without triggering an error
    assert_eq!(ic.advance(1), Err(Fault::MemoryExceeded));

    Ok(())
}

#[test]
fn test_memory_retrieval() -> FaultResult {
    init_logger();

    let mut ic = IntCodeComputer::default();

    ic.store(7, 45)?;
    assert_eq!(ic.mem_read(7)?, 45);

    assert_eq!(ic.mem_read(1), Err(Fault::MissingMemory(0, 1)));
    assert_eq!(ic.mem_read((MEMORY_SIZE + 1).try_into().unwrap()), Err(Fault::MemoryExceeded));

    Ok(())
}

#[test]
fn test_memory_storage() -> FaultResult {
    init_logger();

    let mut ic = IntCodeComputer::default();

    ic.store(0, 100)?;
    assert_eq!(ic.mem_read(0)?, 100);

    assert_eq!(ic.store((MEMORY_SIZE + 1).try_into().unwrap(), 6000), Err(Fault::MemoryExceeded));

    Ok(())
}

#[test]
fn test_halt_checking() -> FaultResult {
    init_logger();

    let mut ic = IntCodeComputer::default();

    // Setup our memory so we can advance through a couple of operation states
    ic.store(0, 1)?;
    ic.store(1, 99)?;
    ic.store(2, 1)?;

    assert!(!ic.is_halted());

    ic.advance(1)?;
    assert!(ic.is_halted());

    ic.advance(1)?;
    assert!(!ic.is_halted());

    Ok(())
}

#[test]
fn test_op_parsing() -> FaultResult {
    init_logger();

    let mut ic = IntCodeComputer::default();

    // Setup our memory so we can advance through a couple of operation states
    ic.store(0, 1)?;
    ic.store(1, 2)?;
    ic.store(2, 3)?;
    ic.store(3, 4)?;
    ic.store(4, 5)?;
    ic.store(5, 6)?;
    ic.store(6, 7)?;
    ic.store(7, 8)?;
    ic.store(8, 99)?;
    ic.store(10, 7500)?;

    assert_eq!(ic.current_op()?, Operation::Add(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Mul(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Input);

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Output(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::JumpIfTrue(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::JumpIfFalse(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::LessThan(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Equals(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Halt);

    ic.advance(1)?;
    assert_eq!(ic.current_op(), Err(Fault::UninitializedOperation(9)));

    ic.advance(1)?;
    assert_eq!(ic.current_op(), Err(Fault::UnknownOperation(10, 7500)));

    Ok(())
}

#[test]
fn test_prog_parsing() {
    init_logger();

    let sample_prog = "1,2,3,4,5";
    let ic = IntCodeComputer::from_str(sample_prog).unwrap();

    assert_eq!(ic.memory_str(), sample_prog);
}

#[test]
fn test_trailing_whitespace() {
    init_logger();

    let sample_prog = "1,2,3,100,0\n";
    let ic = IntCodeComputer::from_str(sample_prog).unwrap();

    assert_eq!(ic.memory_str(), "1,2,3,100,0");
}

#[test]
fn test_addition_step() -> FaultResult {
    init_logger();

    let sample_prog = "1,4,5,6,10,20";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Add(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 4);
    assert_eq!(ic.memory_str(), "1,4,5,6,10,20,30");

    Ok(())
}

#[test]
fn test_multiplication_step() -> FaultResult {
    init_logger();

    let sample_prog = "2,4,5,6,10,20";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Mul(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 4);
    assert_eq!(ic.memory_str(), "2,4,5,6,10,20,200");

    Ok(())
}

#[test]
fn test_input_step() -> FaultResult {
    init_logger();

    let sample_prog = "3,3,99";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    ic.add_input(vec![-832]);
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Input);
    ic.step()?;
    assert_eq!(ic.program_counter(), 2);
    assert_eq!(ic.memory_str(), "3,3,99,-832");

    Ok(())
}

#[test]
fn test_output_step() -> FaultResult {
    init_logger();

    let sample_prog = "4,3,99,9723";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Output(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 2);
    assert_eq!(ic.output(), vec![9723]);

    // Output should clear after being pulled
    assert_eq!(ic.output(), vec![]);

    Ok(())
}

#[test]
fn test_jump_if_true_step() -> FaultResult {
    init_logger();

    let sample_prog = "5,0,5,1000,99,45";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::JumpIfTrue(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 45);

    let sample_prog = "105,0,500,99";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::JumpIfTrue(1));
    ic.step()?;
    assert_eq!(ic.program_counter(), 3);

    Ok(())
}

#[test]
fn test_jump_if_false_step() -> FaultResult {
    init_logger();

    let sample_prog = "106,0,3,8,1,2,3,1,99";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::JumpIfFalse(1));
    ic.step()?;
    assert_eq!(ic.program_counter(), 8);

    let sample_prog = "1006,0,23,99";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::JumpIfFalse(10));
    ic.step()?;
    assert_eq!(ic.program_counter(), 3);

    Ok(())
}

#[test]
fn test_less_than_step() -> FaultResult {
    init_logger();

    let sample_prog = "7,5,6,4,99,2,20";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::LessThan(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 4);
    assert_eq!(ic.memory_str(), "7,5,6,4,1,2,20");

    let sample_prog = "7,5,6,4,99,20,20";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::LessThan(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 4);
    assert_eq!(ic.memory_str(), "7,5,6,4,0,20,20");

    Ok(())
}

#[test]
fn test_equals_step() -> FaultResult {
    init_logger();

    let sample_prog = "1108,10,10,4,99";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Equals(11));
    ic.step()?;
    assert_eq!(ic.program_counter(), 4);
    assert_eq!(ic.memory_str(), "1108,10,10,4,1");

    let sample_prog = "1008,5,5,4,99,100";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Equals(10));
    ic.step()?;
    assert_eq!(ic.program_counter(), 4);
    assert_eq!(ic.memory_str(), "1008,5,5,4,0,100");

    Ok(())
}

#[test]
fn test_halt_step() -> FaultResult {
    init_logger();

    let sample_prog = "99";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Halt);
    ic.step()?;
    assert_eq!(ic.memory_str(), "99");
    assert_eq!(ic.program_counter(), 1);

    Ok(())
}

// This is the test program walked through by the advent challenge
#[test]
fn test_stepping_sample_prog() -> FaultResult {
    init_logger();

    let sample_prog = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.step()?;
    assert_eq!(ic.memory_str(), "1,9,10,70,2,3,11,0,99,30,40,50");
    assert_eq!(ic.program_counter(), 4);

    ic.step()?;
    assert_eq!(ic.memory_str(), "3500,9,10,70,2,3,11,0,99,30,40,50");
    assert_eq!(ic.program_counter(), 8);

    // This is the halt instruction and should also complete successfully, termination of
    // execution is tested via the run() function.
    ic.step()?;

    Ok(())
}

// Test the same program but rather than stepping just run it
#[test]
fn test_running_sample_prog() -> FaultResult {
    init_logger();

    let sample_prog = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.run()?;
    assert_eq!(ic.memory_str(), "3500,9,10,70,2,3,11,0,99,30,40,50");

    Ok(())
}

#[test]
fn test_additional_progs() -> FaultResult {
    init_logger();

    let programs: [(&'static str, &'static str); 4] = [
        ("1,0,0,0,99", "2,0,0,0,99"),
        ("2,3,0,3,99", "2,3,0,6,99"),
        ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
        ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
    ];

    for (prog, result) in programs.iter() {
        let mut ic = IntCodeComputer::from_str(prog)?;
        ic.run()?;
        assert_eq!(ic.memory_str(), result.to_string());
    }

    Ok(())
}

#[test]
fn test_input_output_program() -> FaultResult {
    init_logger();

    let sample_prog = "3,0,4,0,99";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    ic.add_input(vec![673]);

    ic.run()?;
    assert_eq!(ic.output(), vec![673]);

    Ok(())
}

#[test]
fn test_parameter_mode_samples() -> FaultResult {
    init_logger();

    let sample_prog = "1002,4,3,4,33";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.run()?;
    assert_eq!(ic.memory_str(), "1002,4,3,4,99");

    Ok(())
}

#[test]
fn test_jump_instruction_samples1() -> FaultResult {
    init_logger();

    let sample_prog = "3,9,8,9,10,9,4,9,99,-1,8";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.add_input(vec![4]);
    ic.run()?;
    assert_eq!(ic.output(), vec![0]);

    ic.reset();
    ic.add_input(vec![8]);
    ic.run()?;
    assert_eq!(ic.output(), vec![1]);

    Ok(())
}

#[test]
fn test_jump_instruction_samples2() -> FaultResult {
    init_logger();

    let sample_prog = "3,3,1108,-1,8,3,4,3,99";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.add_input(vec![-10]);
    ic.run()?;
    assert_eq!(ic.output(), vec![0]);

    ic.reset();
    ic.add_input(vec![8]);
    ic.run()?;
    assert_eq!(ic.output(), vec![1]);

    Ok(())
}

#[test]
fn test_jump_instruction_samples3() -> FaultResult {
    init_logger();

    let sample_prog = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.add_input(vec![0]);
    ic.run()?;
    assert_eq!(ic.output(), vec![0]);

    ic.reset();
    ic.add_input(vec![129]);
    ic.run()?;
    assert_eq!(ic.output(), vec![1]);

    Ok(())
}

#[test]
fn test_jump_instruction_samples4() -> FaultResult {
    init_logger();

    let sample_prog = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.add_input(vec![0]);
    ic.run()?;
    assert_eq!(ic.output(), vec![0]);

    ic.reset();
    ic.add_input(vec![129]);
    ic.run()?;
    assert_eq!(ic.output(), vec![1]);

    Ok(())
}

#[test]
fn test_jump_instruction_samples5() -> FaultResult {
    init_logger();

    let sample_prog = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.add_input(vec![5]);
    ic.run()?;
    assert_eq!(ic.output(), vec![999]);

    ic.reset();
    ic.add_input(vec![8]);
    ic.run()?;
    assert_eq!(ic.output(), vec![1000]);

    ic.reset();
    ic.add_input(vec![92]);
    ic.run()?;
    assert_eq!(ic.output(), vec![1001]);

    Ok(())
}

#[test]
fn test_system_reset() -> FaultResult {
    init_logger();

    let prog = "1,8,4,1,2,2,1,4,99";
    let mut ic = IntCodeComputer::from_str(&prog)?;

    ic.run()?;
    assert_eq!(ic.memory_str(), "1,101,4,1,404,2,1,4,99");
    assert_eq!(ic.program_counter(), 8);

    ic.reset();
    assert_eq!(ic.memory_str(), prog);
    assert_eq!(ic.program_counter(), 0);

    Ok(())
}
