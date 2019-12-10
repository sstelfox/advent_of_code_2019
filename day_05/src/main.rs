use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use computer::IntCodeComputer;

fn main() {
    let mut in_dat_fh = File::open("./data/input.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
    let mut icc = IntCodeComputer::from_str(&in_dat).unwrap();
    icc.set_input(vec![1]);

    if let Err(err) = icc.run() {
        println!("Running the program encountered and error: {:?}", err);
        std::process::exit(1);
    };

    println!("Output of program was: {:?}", icc.output());
}
