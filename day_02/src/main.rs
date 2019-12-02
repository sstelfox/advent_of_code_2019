use std::fs::File;
use std::io::Read;
use std::str::FromStr;

const MEMORY_SIZE: usize = 100;

struct IntcodeComputer {
    pos: usize,
    memory: [Option<usize>; MEMORY_SIZE],
}

impl IntcodeComputer {
    /// Convert the internal memory representation into the format used by the
    /// Advent examples.
    pub fn memory_str(&self) -> String {
        self.memory.iter().filter_map(|m| m.as_ref()).map(|m| m.to_string()).collect::<Vec<_>>().join(",")
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
    }
}
