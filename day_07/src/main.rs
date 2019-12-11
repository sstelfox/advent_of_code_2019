use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use computer::{Fault, IntCodeComputer};

pub fn amplifier_chain(program: &str, settings: &[isize]) -> Result<isize, Fault> {
    let mut icc = IntCodeComputer::from_str(&program)?;
    let mut signal = 0;

    for val in settings.into_iter() {
        icc.reset();
        icc.add_input(vec![*val, signal]);
        icc.run()?;

        signal = icc.output().into_iter().nth(0).unwrap();
    }

    Ok(signal)
}

pub fn amplifier_feedback_chain(program: &str, settings: &[isize]) -> Result<isize, Fault> {
    let initial_inputs: [isize; 5] = [5, 6, 7, 8, 9];

    let computers: Vec<IntCodeComputer> = initial_inputs
        .into_iter()
        .map(|ii| {
            let mut icc = IntCodeComputer::from_str(&program);
            icc.add_input(vec![ii]);
            icc
        })
        .collect();

    unimplemented!();
}

pub fn is_valid_setting(settings: &[isize]) -> bool {
    // Must have length of 5
    if settings.len() != 5 {
        return false;
    }

    // Must contain each setting (and by proxy, contain it exactly once)
    for i in 0..5 {
        if settings.iter().find(|s| i as isize == **s).is_none() {
            return false;
        }
    }

    true
}

pub fn is_valid_feedback_setting(settings: &[isize]) -> bool {
    // Must have length of 5
    if settings.len() != 5 {
        return false;
    }

    // Must contain each setting (and by proxy, contain it exactly once)
    for i in 5..10 {
        if settings.iter().find(|s| i as isize == **s).is_none() {
            return false;
        }
    }

    true
}

pub fn find_maximum_output(program: &str) -> Result<isize, Fault> {
    let mut amplifier_settings: [isize; 5] = [0; 5];
    let mut max_value = 0;

    loop {
        // We only calculate and update the chain if the settings are valid
        if is_valid_setting(&amplifier_settings) {
            let new_value = amplifier_chain(&program, &amplifier_settings)?;

            if new_value > max_value {
                max_value = new_value;
            }
        }

        for pos in 0..5 {
            amplifier_settings[pos] += 1;

            if amplifier_settings[pos] > 4 {
                // We're at the maximum value for the last position, return whatever we have
                if pos == 4 {
                    return Ok(max_value);
                }

                amplifier_settings[pos] = 0;
            } else {
                break;
            }
        }
    }
}

pub fn find_maximum_feedback_output(program: &str) -> Result<isize, Fault> {
    let mut amplifier_settings: [isize; 5] = [5, 6, 7, 8, 9];
    let mut max_value = 0;

    loop {
        // We only calculate and update the chain if the settings are valid
        if is_valid_feedback_setting(&amplifier_settings) {
            let new_value = amplifier_feedback_chain(&program, &amplifier_settings)?;

            if new_value > max_value {
                max_value = new_value;
            }
        }

        for pos in 0..5 {
            amplifier_settings[pos] += 1;

            if amplifier_settings[pos] > 10 {
                // We're at the maximum value for the last position, return whatever we have
                if pos == 4 {
                    return Ok(max_value);
                }

                amplifier_settings[pos] = 5;
            } else {
                break;
            }
        }
    }
}

pub fn get_program() -> String {
    let mut in_dat_fh = File::open("./data/input.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    in_dat
}

fn main() {
    let prog = get_program();

    let max_value = match find_maximum_output(&prog) {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error running the program: {:?}", err);
            std::process::exit(1);
        }
    };
    println!("Maximum value for input program was: {}", max_value);

    let max_feedback_value = match find_maximum_feedback_output(&prog) {
        Ok(val) => val,
        Err(err) => {
            println!("There was an error running the program: {:?}", err);
            std::process::exit(1);
        }
    };
    println!(
        "Maximum feedback value for input program was: {}",
        max_feedback_value
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    type FaultResult = Result<(), computer::Fault>;

    #[test]
    fn test_sample_program_chains1() -> FaultResult {
        let sample_prog = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let output = amplifier_chain(&sample_prog, &vec![4, 3, 2, 1, 0])?;
        assert_eq!(output, 43210);

        Ok(())
    }

    #[test]
    fn test_sample_program_chains2() -> FaultResult {
        let sample_prog =
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let output = amplifier_chain(&sample_prog, &vec![0, 1, 2, 3, 4])?;
        assert_eq!(output, 54321);

        Ok(())
    }

    #[test]
    fn test_sample_program_chains3() -> FaultResult {
        let sample_prog = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let output = amplifier_chain(&sample_prog, &vec![1, 0, 4, 3, 2])?;
        assert_eq!(output, 65210);

        Ok(())
    }

    #[test]
    fn test_find_maximum_output1() -> FaultResult {
        let sample_prog = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let output = find_maximum_output(&sample_prog)?;
        assert_eq!(output, 43210);

        Ok(())
    }

    #[test]
    fn test_find_maximum_output2() -> FaultResult {
        let sample_prog =
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let output = find_maximum_output(&sample_prog)?;
        assert_eq!(output, 54321);

        Ok(())
    }

    #[test]
    fn test_find_maximum_output3() -> FaultResult {
        let sample_prog = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let output = find_maximum_output(&sample_prog)?;
        assert_eq!(output, 65210);

        Ok(())
    }
}
