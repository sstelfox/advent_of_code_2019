pub fn check_numeric_rules(num: usize) -> Result<bool, &'static str> {
    let num_list = split_numeric(num)?;
    let mut found_double = false;

    for (i, num) in num_list.iter().enumerate() {
        if i < 5 {
            if *num > num_list[i + 1] {
                return Ok(false);
            }

            if found_double {
                continue;
            }

            if *num == num_list[i + 1] {
                found_double = true;
            }
        }
    }

    Ok(found_double)
}

pub fn check_extended_numeric_rules(num: usize) -> Result<bool, &'static str> {
    let num_list = split_numeric(num)?;
    let mut double_list: Vec<u8> = Vec::new();
    let mut current_run: Option<u8> = None;

    for (i, num) in num_list.iter().enumerate() {
        if i < 5 {
            if *num > num_list[i + 1] {
                return Ok(false);
            }

            if *num == num_list[i + 1] {
                if current_run == Some(*num) {
                    // We're in a run with more than one of the same type, but we only care if the
                    // current double matches our number.
                    if let Some(double) = double_list.pop() {
                        // If the last found double wasn't our number put it back...
                        if double != *num {
                            double_list.push(*num);
                        }
                    }
                } else {
                    current_run = Some(*num);
                    double_list.push(*num);
                }
            }
        }
    }

    Ok(!double_list.is_empty())
}

pub fn split_numeric(num: usize) -> Result<[u8; 6], &'static str> {
    // We can only handle six digit numbers
    if num < 100_000 || num >= 1_000_000 {
        return Err("value is outside the correct range");
    }

    // Should probably do this recursively but its simple enough to just hard code these...
    let digits: [u8; 6] = [
        (num / 100_000) as u8,
        (num / 10_000 % 10) as u8,
        (num / 1_000 % 10) as u8,
        (num / 100 % 10) as u8,
        (num / 10 % 10) as u8,
        (num % 10) as u8,
    ];

    Ok(digits)
}

fn main() {
    let mut total_checked = 0;
    let mut match_count = 0;
    let mut extended_match_count = 0;

    // Note: The last number is not included in the range and the problem doesn't specify whether
    // this needs to be included or not. It doesn't matter in this case though as the first and
    // final digit both fail the validation rules.
    for num in 153_517..630_395 {
        total_checked += 1;

        if check_numeric_rules(num).unwrap() {
            match_count += 1;
        }

        if check_extended_numeric_rules(num).unwrap() {
            extended_match_count += 1;
        }
    }

    println!("In the given range there were basic {} matches out of {}", match_count, total_checked);
    println!("In the given range there were extended {} matches out of {}", extended_match_count, total_checked);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_rule_checker() {
        assert!(check_numeric_rules(111_111).unwrap());
        assert!(!check_numeric_rules(223_450).unwrap());
        assert!(!check_numeric_rules(123_789).unwrap());

        assert!(check_numeric_rules(1_000).is_err());
        assert!(check_numeric_rules(1_000_000).is_err());
    }

    #[test]
    fn test_extended_numeric_rule_checker() {
        assert!(check_extended_numeric_rules(112_233).unwrap());
        assert!(check_extended_numeric_rules(111_122).unwrap());

        assert!(!check_extended_numeric_rules(111_111).unwrap());
        assert!(!check_extended_numeric_rules(123_444).unwrap());
        assert!(!check_extended_numeric_rules(223_450).unwrap());
        assert!(!check_extended_numeric_rules(123_789).unwrap());

        assert!(check_numeric_rules(1_000).is_err());
        assert!(check_numeric_rules(1_000_000).is_err());
    }

    #[test]
    fn test_split_numeric() {
        assert!(split_numeric(1_000).is_err());
        assert!(split_numeric(1_000_000).is_err());

        assert_eq!(split_numeric(123_456).unwrap(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(split_numeric(783_100).unwrap(), [7, 8, 3, 1, 0, 0]);
    }
}
