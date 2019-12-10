use super::*;

type FaultResult = Result<(), Fault>;

#[test]
fn test_advancing() -> FaultResult {
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
        original_input: Vec::new(),

        memory: [None; MEMORY_SIZE],
        original_memory: [None; MEMORY_SIZE],

        output: Vec::new(),
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
    let mut ic = IntCodeComputer::default();

    ic.store(7, 45)?;
    assert_eq!(ic.retrieve(7)?, 45);

    assert_eq!(ic.retrieve(1), Err(Fault::MissingMemory(0, 1)));
    assert_eq!(ic.retrieve((MEMORY_SIZE + 1).try_into().unwrap()), Err(Fault::MemoryExceeded));

    Ok(())
}

#[test]
fn test_memory_storage() -> FaultResult {
    let mut ic = IntCodeComputer::default();

    ic.store(0, 100)?;
    assert_eq!(ic.retrieve(0)?, 100);

    assert_eq!(ic.store((MEMORY_SIZE + 1).try_into().unwrap(), 6000), Err(Fault::MemoryExceeded));

    Ok(())
}

#[test]
fn test_halt_checking() -> FaultResult {
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
    let mut ic = IntCodeComputer::default();

    // Setup our memory so we can advance through a couple of operation states
    ic.store(0, 1)?;
    ic.store(1, 2)?;
    ic.store(2, 3)?;
    ic.store(3, 4)?;
    ic.store(4, 99)?;
    ic.store(6, 7500)?;

    assert_eq!(ic.current_op()?, Operation::Add(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Mul(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Input(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Output(0));

    ic.advance(1)?;
    assert_eq!(ic.current_op()?, Operation::Halt);

    ic.advance(1)?;
    assert_eq!(ic.current_op(), Err(Fault::UninitializedOperation(5)));

    ic.advance(1)?;
    assert_eq!(ic.current_op(), Err(Fault::UnknownOperation(6, 7500)));

    Ok(())
}

#[test]
fn test_prog_parsing() {
    let sample_prog = "1,2,3,4,5";
    let ic = IntCodeComputer::from_str(sample_prog).unwrap();

    assert_eq!(ic.memory_str(), sample_prog);
}

#[test]
fn test_trailing_whitespace() {
    let sample_prog = "1,2,3,100,0\n";
    let ic = IntCodeComputer::from_str(sample_prog).unwrap();

    assert_eq!(ic.memory_str(), "1,2,3,100,0");
}

#[test]
fn test_addition_step() -> FaultResult {
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
    let sample_prog = "3,3,99";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    ic.set_input(vec![-832]);
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Input(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 2);
    assert_eq!(ic.memory_str(), "3,3,99,-832");

    Ok(())
}

#[test]
fn test_output_step() -> FaultResult {
    let sample_prog = "4,3,99,9723";

    let mut ic = IntCodeComputer::from_str(sample_prog)?;
    assert_eq!(ic.memory_str(), sample_prog);

    assert_eq!(ic.current_op()?, Operation::Output(0));
    ic.step()?;
    assert_eq!(ic.program_counter(), 2);
    assert_eq!(ic.output(), vec![9723]);

    Ok(())
}

#[test]
fn test_halt_step() -> FaultResult {
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
    let sample_prog = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut ic = IntCodeComputer::from_str(sample_prog)?;

    ic.run()?;
    assert_eq!(ic.memory_str(), "3500,9,10,70,2,3,11,0,99,30,40,50");

    Ok(())
}

#[test]
fn test_additional_progs() -> FaultResult {
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
fn test_system_reset() -> FaultResult {
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
