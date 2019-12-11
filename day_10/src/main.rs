use std::fs::File;
use std::io::Read;

fn main() {
    let mut in_dat_fh = File::open("./data/input.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();
}

#[cfg(test)]
mod tests {}
