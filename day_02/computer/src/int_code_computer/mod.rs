use std::str::FromStr;

/// The amount of RAM the IntCodeComputer has. I could change the implementation to allow for
/// arbitrary sized inputs by using a Vec<_> instead, but this feels more appropriate for the task.
pub const MEMORY_SIZE: usize = 200;

/// This error state encapsulates the various ways a program run on the IntCodeComputer can fail
/// and would generally be considered a hardware fault if it happened on a real machine.
#[derive(Debug, PartialEq)]
pub enum Fault {
    MemoryExceeded,
    MissingMemory(usize, usize),
    ProgramTooBig(usize),
    UninitializedOperation(usize),
    UnknownOperation(usize, usize),
}

/// An IntCodeComputer emulator as defined in the day 2 segment of the 2019 Advent of Code.
pub struct IntCodeComputer {
    pc: usize,
    memory: [Option<usize>; MEMORY_SIZE],
    original_memory: [Option<usize>; MEMORY_SIZE],
}

impl IntCodeComputer {
    /// Advances the current program counter the provided amount. In part 1 of day 2, where this
    /// was initially specified it always advanced a fix amount (4). Part 2 expanded on this
    /// indicating that it should advance 1 + (number of parameters operator takes). This is still
    /// 4 for Add and Mul, but was specified to be 1 for Halt. Since it is likely that this will
    /// come up later, I went ahead and implemented it.
    ///
    /// It's important to note that this allows exactly one invalid index intentionally (self.pc ==
    /// MEMORY_SIZE). This is not a valid memory address but allows Halt to be the final
    /// instruction up against our memory limit (which I did define arbitrarily).
    pub fn advance(&mut self, amount: usize) -> Result<(), Fault> {
        let new_pc = self.pc + amount;

        // The less than here is intentional. We want to allow the program counter to be
        // incremented 1 beyond the memory size so the last valid instruction is allowed to be a
        // Halt. Any other instruction should still throw a memory error...
        if new_pc > MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        self.pc = new_pc;
        Ok(())
    }

    /// Decodes the operation pointed to by the program counter. Will fault if the operation is
    /// unknown or if the program as entered uninitialized memory.
    pub fn current_op(&self) -> Result<Operation, Fault> {
        if self.pc >= MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        match self.memory[self.pc] {
            Some(op) => match op {
                1 => Ok(Operation::Add),
                2 => Ok(Operation::Mul),
                99 => Ok(Operation::Halt),
                _ => Err(Fault::UnknownOperation(self.pc, op)),
            },
            None => Err(Fault::UninitializedOperation(self.pc)),
        }
    }

    /// Initialize a new IntCodeComputer emulator with the provided memory. This must be a slice
    /// equal in size to `MEMORY_SIZE`.
    pub fn new(memory: [Option<usize>; MEMORY_SIZE]) -> Self {
        IntCodeComputer {
            pc: 0,
            memory,
            original_memory: memory,
        }
    }

    /// The advent challenge refers to this as the instruction pointer the computer is currently
    /// at, but I prefer the more traditional program counter or `pc`. This retrieves the location
    /// in memory the program is currently executing or about to execute.
    pub fn program_counter(&self) -> usize {
        self.pc
    }

    /// A helper function for determining whether or not the machine has hit a valid halt state.
    /// This will not trip for errors, instead the result state of a step() should be checked to
    /// see if an error occured. Attempted execution after an error or halt occurs is undefined
    /// behavior.
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
        self.memory
            .iter()
            .filter_map(|m| m.as_ref())
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Resets the computer to the initial state it was created with and resets the program counter
    /// to 0.
    pub fn reset(&mut self) {
        self.memory = self.original_memory;
        self.pc = 0;
    }

    /// Safely returns the value stored at the provided memory address. Will fault in the event of
    /// invalid addresses or uninitialized memory.
    pub fn retrieve(&self, address: usize) -> Result<usize, Fault> {
        if address >= MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        match self.memory[address] {
            Some(val) => Ok(val),
            None => Err(Fault::MissingMemory(self.pc, address)),
        }
    }

    /// Run the computer until it reaches a halt (success), or a fault (failure). If there was a
    /// more complicated instruction set that involved jumps I would likely want to limit the
    /// runtime of this to a certain number of instructions to ensure it always completed, but as
    /// it stands it can at most execute MEMORY_SIZE / 4 instructions before exiting.
    pub fn run(&mut self) -> Result<(), Fault> {
        loop {
            self.step()?;

            if self.is_halted() {
                return Ok(());
            }
        }
    }

    /// Steps the state of the computer by performing one operation and advancing the program
    /// counter an appropriate amount. Will fault if the current program counter, any parameters,
    /// or target addresses are outside of the valid memory range or are uninitialized.
    pub fn step(&mut self) -> Result<(), Fault> {
        // Note: This needs to be stored here. After performing an operation the operation that the
        // current program counter is pointing at may have been modified. We need the original
        // instruction to ensure we correctly advance to the next program state.
        let current_op = self.current_op()?;

        match current_op {
            Operation::Add => {
                let left_addr = self.retrieve(self.pc + 1)?;
                let right_addr = self.retrieve(self.pc + 2)?;
                let dest_addr = self.retrieve(self.pc + 3)?;

                let left_val = self.retrieve(left_addr)?;
                let right_val = self.retrieve(right_addr)?;

                self.store(dest_addr, left_val + right_val)?;
            }
            Operation::Mul => {
                let left_addr = self.retrieve(self.pc + 1)?;
                let right_addr = self.retrieve(self.pc + 2)?;
                let dest_addr = self.retrieve(self.pc + 3)?;

                let left_val = self.retrieve(left_addr)?;
                let right_val = self.retrieve(right_addr)?;

                self.store(dest_addr, left_val * right_val)?;
            }
            _ => (),
        }

        // Note: Depending on the instructions added in the future I may need to move this into the
        // individual operation processing blocks...
        self.advance(current_op.instruction_size())?;

        Ok(())
    }

    /// Safely stores the provided value at the provided address. This will fault only if the
    /// memory address is invalid.
    pub fn store(&mut self, address: usize, value: usize) -> Result<(), Fault> {
        if address >= MEMORY_SIZE {
            return Err(Fault::MemoryExceeded);
        }

        self.memory[address] = Some(value);
        Ok(())
    }
}

impl Default for IntCodeComputer {
    /// This is a pretty boring method. It creates an empty emulator with no initialized memory.
    /// This can be useful for testing but would be tedious to build up a machine using `store()`
    /// alone. Resetting this will go back to the default uninitialized state.
    fn default() -> Self {
        IntCodeComputer {
            pc: 0,
            memory: [None; MEMORY_SIZE],
            original_memory: [None; MEMORY_SIZE],
        }
    }
}

impl FromStr for IntCodeComputer {
    type Err = Fault;

    /// This parses the official Advent of Code 2019 program code for IntCodeComputer as defined up
    /// to the end of day 2 and returns an instance of the emulator that can be run. This expects
    /// only positive integer numbers on a single line separated by spaces.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_mem: Vec<Option<usize>> = s
            .trim()
            .split(',')
            .map(|s| Some(s.parse::<usize>().unwrap()))
            .collect();
        if raw_mem.len() > MEMORY_SIZE {
            return Err(Fault::ProgramTooBig(raw_mem.len()));
        }

        let mut memory: [Option<usize>; MEMORY_SIZE] = [None; MEMORY_SIZE];
        memory[..raw_mem.len()].copy_from_slice(&raw_mem);

        Ok(IntCodeComputer::new(memory))
    }
}

/// This specifies the valid instruction set for the IntCodeComputer as defined by the 2019 Advent
/// Code calendar up to day 2.
#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    Halt,
}

impl Operation {
    /// Instructions have varying widths. This returns the amount of memory they take up so they
    /// can be appropriately jumped over to the next instruction.
    pub fn instruction_size(&self) -> usize {
        match *self {
            Operation::Add => 4,
            Operation::Mul => 4,
            Operation::Halt => 1,
        }
    }
}

#[cfg(test)]
mod tests;
