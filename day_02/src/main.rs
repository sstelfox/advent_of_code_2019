use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use computer::IntCodeComputer;

fn main() {
    let mut in_dat_fh = File::open("./data/input_02.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let mut icc = IntCodeComputer::from_str(&in_dat).unwrap();

    // The instructions indicate to make these replacments before running
    icc.store(1, 12).unwrap();
    icc.store(2, 2).unwrap();

    if let Err(err) = icc.run() {
        println!("Program crashed with error: {:?}", err);
    };

    println!("Answer to step 1 is: {}", icc.mem_read(0).unwrap());
    println!("Brute force searching the answer to step 2...");

    // Alright so there are two possibilities for how I could go about finding the answer to step
    // 2. The simple and straight forward is brute forcing the two values. They're both between
    // 0-99 which means there is only 10k possibilities and Rust is very fast here. A more "fun"
    // way to solve this would be to attempt to reverse the execution of the computer. This is
    // still possible because there are no jumps only linear advancement, the only failure
    // possibility here is if one of the opcodes got overwritten by the program... which is
    // possible... Nah I'm just going to bruteforce it.
    for noun in 0..100 {
        for verb in 0..100 {
            icc.reset();

            icc.store(1, noun).unwrap();
            icc.store(2, verb).unwrap();

            icc.run().unwrap();

            if icc.mem_read(0).unwrap() == 19_690_720 {
                println!("Found a valid answer: {:0>2}{:0>2}", noun, verb);
            }
        }
    }
}
