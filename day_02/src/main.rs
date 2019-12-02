use std::fs::File;
use std::io::Read;
use std::str::FromStr;

const MEMORY_SIZE: usize = 100;

pub struct IntcodeComputer {
    pos: usize,
    memory: [Option<usize>; MEMORY_SIZE],
}

impl IntcodeComputer {
    pub fn advance(&mut self) -> Result<(), String> {
        let new_pos = self.pos + 4;
        if new_pos > MEMORY_SIZE {
            return Err("Unable to advance past the end of available memory.".to_string());
        }

        self.pos = new_pos;
        Ok(())
    }

    pub fn current_op(&self) -> Result<Operation, OperationError> {
        match self.memory[self.pos] {
            Some(op) => match op {
                1 => Ok(Operation::Add),
                2 => Ok(Operation::Mul),
                99 => Ok(Operation::Halt),
                _ => Err(OperationError::UnknownOperation(op)),
            },
            None => Err(OperationError::UninitializedOperation),
        }
    }

    pub fn current_pos(&self) -> usize {
        self.pos
    }

    pub fn is_halted(&self) -> bool {
        let c_op = self.current_op();
        c_op.is_err() || c_op == Ok(Operation::Halt)
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

    /// Steps the state of the computer performing one operation. If the 
    pub fn step(&mut self) -> Result<(), String> {

        Ok(())
    }
}

impl FromStr for IntcodeComputer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_mem: Vec<Option<usize>> = s.trim().split(',').map(|s| Some(s.parse::<usize>().unwrap()) ).collect();
        if raw_mem.len() > MEMORY_SIZE {
            return Err(format!("parsed memory was larger than the computer can support: {} vs {}", raw_mem.len(), MEMORY_SIZE));
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

#[derive(Debug, PartialEq)]
pub enum OperationError {
    MissingArgument,
    MissingDestination,
    UninitializedOperation,
    UnknownOperation(usize),
}

fn main() {
    let mut in_dat_fh = File::open("./data/input_02.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let _ic = IntcodeComputer::from_str(&in_dat);
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
    fn test_halt_checking() {
        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];

        // Setup our memory so we can advance through a couple of operation states
        memory[0] = Some(1);
        memory[4] = Some(99);
        memory[8] = None;
        memory[12] = Some(45);
        memory[16] = Some(1);

        let mut ic = IntcodeComputer { pos: 0, memory: memory };
        assert!(!ic.is_halted());

        ic.advance().unwrap();
        assert!(ic.is_halted());

        ic.advance().unwrap();
        assert!(ic.is_halted());

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
        assert_eq!(ic.current_op(), Err(OperationError::UninitializedOperation));

        ic.advance().unwrap();
        assert_eq!(ic.current_op(), Err(OperationError::UnknownOperation(7500)));
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
    fn test_sample_prog1() {
        let sample_prog = "1,9,10,3,2,3,11,0,99,30,40,50";
        let _ic = IntcodeComputer::from_str(sample_prog).unwrap();
        // TODO: test stepping through the program
    }
}