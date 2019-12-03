use std::fs::File;
use std::io::Read;
use std::str::FromStr;

const MEMORY_SIZE: usize = 200;

#[derive(Debug, PartialEq)]
pub enum Fault {
    MemoryExceeded,
    MissingMemory(usize, usize),
    ProgramTooBig(usize),
    UninitializedOperation(usize),
    UnknownOperation(usize, usize),
}

pub struct IntcodeComputer {
    pos: usize,
    memory: [Option<usize>; MEMORY_SIZE],
}

impl IntcodeComputer {
    pub fn advance(&mut self) -> Result<(), Fault> {
        let new_pos = self.pos + 4;
        if new_pos > MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        self.pos = new_pos;
        Ok(())
    }

    pub fn current_op(&self) -> Result<Operation, Fault> {
        match self.memory[self.pos] {
            Some(op) => match op {
                1 => Ok(Operation::Add),
                2 => Ok(Operation::Mul),
                99 => Ok(Operation::Halt),
                _ => Err(Fault::UnknownOperation(self.pos, op)),
            },
            None => Err(Fault::UninitializedOperation(self.pos)),
        }
    }

    pub fn current_pos(&self) -> usize {
        self.pos
    }

    pub fn is_halted(&self) -> bool {
        self.current_op() == Ok(Operation::Halt)
    }

    /// Convert the internal memory representation into the format used by the Advent examples.
    ///
    /// The challenge doesn't specify the value of uninitialized memory or have a representation of
    /// it, thus intermediate uninitialized values are undefined behavior for this output. This
    /// implementation choose to preserve ordering in the event of this undefined behavior but does
    /// not preserve memory addresses (uninitialized memory is ignored for this output).
    ///
    /// Thus if the memory state was `[Some(1), Some(2), None, Some(3)]` the output would be
    /// reflected as `1,2,3` where the last value has moved from the fourth index to the third.
    pub fn memory_str(&self) -> String {
        self.memory.iter().filter_map(|m| m.as_ref()).map(|m| m.to_string()).collect::<Vec<_>>().join(",")
    }

    /// Safely returns the value stored at the provided address.
    pub fn retrieve(&self, position: usize) -> Result<usize, Fault> {
        if position > MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        match self.memory[position] {
            Some(val) => Ok(val),
            None => Err(Fault::MissingMemory(self.pos, position)),
        }
    }

    /// Run the computer until it reaches a halt (success), or a fault (failure). If there was a
    /// more complicated instruction set that involved jumps I would likely want to limit the
    /// runtime of this to a certain number of instructions to ensure it always completed, but as
    /// it stands it can at most execute MEMORY_SIZE / 4 instructions before exiting.
    pub fn run(&mut self) -> Result<(), Fault> {
        loop {
            if self.is_halted() {
                return Ok(());
            }

            self.step()?;
        }
    }

    /// Steps the state of the computer performing one operation. If the 
    pub fn step(&mut self) -> Result<(), Fault> {
        match self.current_op()? {
            Operation::Add => {
                let left_addr = self.retrieve(self.pos + 1)?;
                let right_addr = self.retrieve(self.pos + 2)?;
                let dest_addr = self.retrieve(self.pos + 3)?;

                let left_val = self.retrieve(left_addr)?;
                let right_val = self.retrieve(right_addr)?;

                self.store(dest_addr, left_val + right_val)?;
            },
            Operation::Mul => {
                let left_addr = self.retrieve(self.pos + 1)?;
                let right_addr = self.retrieve(self.pos + 2)?;
                let dest_addr = self.retrieve(self.pos + 3)?;

                let left_val = self.retrieve(left_addr)?;
                let right_val = self.retrieve(right_addr)?;

                self.store(dest_addr, left_val * right_val)?;
            },
            _ => (),
        }

        // Not sure if this is undefined behavior or intentional. The challenge specifically
        // states:
        //
        // > Once you're done processing an opcode, move to the next one by stepping forward 4
        // > positions.
        //
        // Halt is a valid operation and we will have successfully processed it at this point so
        // according to the above we should advance the program counter even if it is a halt
        // instruction...
        self.advance()?;

        Ok(())
    }

    pub fn store(&mut self, position: usize, value: usize) -> Result<(), Fault> {
        if position > MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        self.memory[position] = Some(value);
        Ok(())
    }
}

impl FromStr for IntcodeComputer {
    type Err = Fault;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_mem: Vec<Option<usize>> = s.trim().split(',').map(|s| Some(s.parse::<usize>().unwrap()) ).collect();
        if raw_mem.len() > MEMORY_SIZE {
            return Err(Fault::ProgramTooBig(raw_mem.len()));
        }

        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];
        memory[..raw_mem.len()].copy_from_slice(&raw_mem);

        let ic = IntcodeComputer {
            pos: 0,
            memory: memory,
        };

        Ok(ic)
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    Halt,
}

fn main() {
    let mut in_dat_fh = File::open("./data/input_02.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let mut ic = IntcodeComputer::from_str(&in_dat).unwrap();

    // The instructions indicate to make these replacments before running
    ic.store(1, 12).unwrap();
    ic.store(2, 2).unwrap();

    match ic.run() {
        Ok(_) => println!("Program executed successfully."),
        Err(err) => println!("Program crashed with error: {:?}", err),
    }

    println!("Final state was: {}", ic.memory_str());
    println!("Answer to step 1 is: {}", ic.retrieve(0).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_advancing() {
        // Advancing doesn't care about the state of the memory we moved into, so the entire
        // program can be empty during this test.
        let mut ic = IntcodeComputer {
            pos: 0,
            memory: [None; MEMORY_SIZE],
        };

        assert_eq!(ic.advance(), Ok(()));
        assert_eq!(ic.current_pos(), 4);
        assert_eq!(ic.advance(), Ok(()));
        assert_eq!(ic.current_pos(), 8);

        // Also ensure we can't advance past the end of our memory without triggering an error
        let mut ic = IntcodeComputer {
            pos: MEMORY_SIZE - 1,
            memory: [None; MEMORY_SIZE],
        };

        assert!(ic.advance().is_err());
    }

    #[test]
    fn test_memory_retrieval() {
        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];
        memory[7] = Some(45);

        let ic = IntcodeComputer { pos: 0, memory: memory };

        assert_eq!(ic.retrieve(7), Ok(45));
        assert_eq!(ic.retrieve(1), Err(Fault::MissingMemory(0, 1)));
        assert_eq!(ic.retrieve(MEMORY_SIZE + 1), Err(Fault::MemoryExceeded));
    }

    #[test]
    fn test_memory_storage() {
        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];
        let mut ic = IntcodeComputer { pos: 0, memory: memory };

        assert_eq!(ic.store(0, 100), Ok(()));
        assert_eq!(ic.retrieve(0), Ok(100));
        assert_eq!(ic.store(MEMORY_SIZE + 1, 6000), Err(Fault::MemoryExceeded));
    }

    #[test]
    fn test_halt_checking() {
        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];

        // Setup our memory so we can advance through a couple of operation states
        memory[0] = Some(1);
        memory[4] = Some(99);
        memory[16] = Some(1);

        let mut ic = IntcodeComputer { pos: 0, memory: memory };
        assert!(!ic.is_halted());

        ic.advance().unwrap();
        assert!(ic.is_halted());

        ic.advance().unwrap();
        assert!(!ic.is_halted());
    }

    #[test]
    fn test_op_parsing() {
        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];

        // Setup our memory so we can advance through a couple of operation states
        memory[0] = Some(1);
        memory[4] = Some(2);
        memory[8] = Some(99);
        memory[12] = None;
        memory[16] = Some(7500);

        let mut ic = IntcodeComputer { pos: 0, memory: memory };
        assert_eq!(ic.current_op(), Ok(Operation::Add));

        ic.advance().unwrap();
        assert_eq!(ic.current_op(), Ok(Operation::Mul));

        ic.advance().unwrap();
        assert_eq!(ic.current_op(), Ok(Operation::Halt));

        ic.advance().unwrap();
        assert_eq!(ic.current_op(), Err(Fault::UninitializedOperation(12)));

        ic.advance().unwrap();
        assert_eq!(ic.current_op(), Err(Fault::UnknownOperation(16, 7500)));
    }

    #[test]
    fn test_prog_parsing() {
        let sample_prog = "1,2,3,4,5";
        let ic = IntcodeComputer::from_str(sample_prog).unwrap();

        assert_eq!(ic.memory_str(), sample_prog);
    }

    #[test]
    fn test_trailing_whitespace() {
        let sample_prog = "1,2,3,100,0\n";
        let ic = IntcodeComputer::from_str(sample_prog).unwrap();

        assert_eq!(ic.memory_str(), "1,2,3,100,0");
    }

    #[test]
    fn test_addition_step() {
        let sample_prog = "1,4,5,6,10,20";

        let mut ic = IntcodeComputer::from_str(sample_prog).unwrap();
        assert_eq!(ic.memory_str(), sample_prog);

        assert_eq!(ic.current_op(), Ok(Operation::Add));
        assert_eq!(ic.step(), Ok(()));
        assert_eq!(ic.current_pos(), 4);
        assert_eq!(ic.memory_str(), "1,4,5,6,10,20,30");
    }

    #[test]
    fn test_multiplication_step() {
        let sample_prog = "2,4,5,6,10,20";

        let mut ic = IntcodeComputer::from_str(sample_prog).unwrap();
        assert_eq!(ic.memory_str(), sample_prog);

        assert_eq!(ic.current_op(), Ok(Operation::Mul));
        assert_eq!(ic.step(), Ok(()));
        assert_eq!(ic.current_pos(), 4);
        assert_eq!(ic.memory_str(), "2,4,5,6,10,20,200");
    }

    #[test]
    fn test_halt_step() {
        let sample_prog = "99";

        let mut ic = IntcodeComputer::from_str(sample_prog).unwrap();
        assert_eq!(ic.memory_str(), sample_prog);

        assert_eq!(ic.current_op(), Ok(Operation::Halt));
        assert_eq!(ic.step(), Ok(()));
        assert_eq!(ic.memory_str(), "99");
        assert_eq!(ic.current_pos(), 4);
    }

    // This is the test program walked through by the advent challenge
    #[test]
    fn test_stepping_sample_prog() {
        let sample_prog = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut ic = IntcodeComputer::from_str(sample_prog).unwrap();

        assert_eq!(ic.step(), Ok(()));
        assert_eq!(ic.memory_str(), "1,9,10,70,2,3,11,0,99,30,40,50");
        assert_eq!(ic.current_pos(), 4);

        assert_eq!(ic.step(), Ok(()));
        assert_eq!(ic.memory_str(), "3500,9,10,70,2,3,11,0,99,30,40,50");
        assert_eq!(ic.current_pos(), 8);

        // This is the halt instruction and should also complete successfully, termination of
        // execution is tested via the run() function.
        assert_eq!(ic.step(), Ok(()));
    }

    // Test the same program but rather than stepping just run it
    #[test]
    fn test_running_sample_prog() {
        let sample_prog = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut ic = IntcodeComputer::from_str(sample_prog).unwrap();

        assert_eq!(ic.run(), Ok(()));
        assert_eq!(ic.memory_str(), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }

    #[test]
    fn test_additional_progs() {
        let programs: [(&'static str, &'static str); 4] = [
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ];

        for (prog, result) in programs.iter() {
            let mut ic = IntcodeComputer::from_str(prog).unwrap();

            assert_eq!(ic.run(), Ok(()));
            assert_eq!(ic.memory_str(), result.to_string());
        }
    }
}
