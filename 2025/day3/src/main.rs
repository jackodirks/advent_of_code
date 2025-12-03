use std::fs;

fn line_to_digit_vec(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn find_largest_digit_and_pos_in_list(digit_slice: &[u32]) -> (u32, usize) {
    let mut hi_val = 0;
    let mut hi_val_pos = 0;
    for (pos, val) in digit_slice.iter().enumerate() {
        if *val > hi_val {
            hi_val = *val;
            hi_val_pos = pos;
        }
    }
    (hi_val, hi_val_pos)
}

fn find_largest_joltage(battery_bank : &[u32], battery_count: usize) -> u64 {
    let mut retval = 0;
    let mut slice_base = 0;
    for index in 0..battery_count {
        let (val, pos) = find_largest_digit_and_pos_in_list(&battery_bank[slice_base..battery_bank.len() - (battery_count - 1 - index)]);
        retval *= 10;
        retval += val as u64;
        slice_base += pos + 1;
    }
    retval
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut total_joltage_with_2_batteries = 0;
    let mut total_joltage_with_12_batteries = 0;
    let battery_banks = contents.trim().split("\n");
    for bank in battery_banks {
        total_joltage_with_2_batteries += find_largest_joltage(&line_to_digit_vec(bank), 2);
        total_joltage_with_12_batteries += find_largest_joltage(&line_to_digit_vec(bank), 12);
    }
    println!("Day 3, part 1: {total_joltage_with_2_batteries}");
    println!("Day 3, part 2: {total_joltage_with_12_batteries}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1212_to_digit_vec() {
        let input = "1212";
        let expect = vec![1,2,1,2];
        let output = line_to_digit_vec(input);
        assert_eq!(output, expect)
    }

    #[test]
    fn parse_1234_to_digit_vec() {
        let input = "1234";
        let expect = vec![1,2,3,4];
        let output = line_to_digit_vec(input);
        assert_eq!(output, expect)
    }

    #[test]
    fn largest_joltage_of_930_with_2_batteries_is_93() {
        let input = vec![9,3,0];
        let battery_count = 2;
        let expect = 93;
        let output = find_largest_joltage(&input, battery_count);
        assert_eq!(output, expect)
    }

    #[test]
    fn largest_joltage_of_19_with_2_batteries_is_19() {
        let input = vec![1,9];
        let battery_count = 2;
        let expect = 19;
        let output = find_largest_joltage(&input, battery_count);
        assert_eq!(output, expect)
    }

    #[test]
    fn largest_joltage_of_987654321111111_with_2_batteries_is_98() {
        let input = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let battery_count = 2;
        let expect = 98;
        let output = find_largest_joltage(&input, battery_count);
        assert_eq!(output, expect)
    }

    #[test]
    fn largest_digit_and_pos_in_7899_is_9_and_2() {
        let input = vec![7,8,9,9];
        let expected_digit = 9;
        let expected_pos = 2;
        let output = find_largest_digit_and_pos_in_list(&input);
        assert_eq!(output, (expected_digit, expected_pos))
    }

    #[test]
    fn largest_joltage_of_987654321111111_with_3_batteries_is_987() {
        let input = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        let battery_count = 3;
        let expect = 987;
        let output = find_largest_joltage(&input, battery_count);
        assert_eq!(output, expect)
    }

    #[test]
    fn largest_joltage_of_987654321111111_with_12_batteries_is_987654321111() {
        let input = line_to_digit_vec("987654321111111");
        let battery_count = 12;
        let expect = 987654321111;
        let output = find_largest_joltage(&input, battery_count);
        assert_eq!(output, expect)
    }
}
