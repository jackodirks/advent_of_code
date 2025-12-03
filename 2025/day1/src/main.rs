mod dial;

use std::fs;
use dial::*;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let parts = contents.split("\n");
    let mut running_number: i32 = 50;
    let mut zero_count: i32 = 0;
    let mut zero_crossings : i32 = 0;
    for part in parts {
        if part.is_empty() {
            continue;
        }
        let zero_crossings_per_part = count_zero_crossings(running_number, part);
        zero_crossings += zero_crossings_per_part;
        running_number = new_dial_value(running_number, part);
        if running_number == 0 {
            zero_count += 1;
        }
    }
    println!("Day 1, part 1 answer: {zero_count}");
    let part_2_answer = zero_count + zero_crossings;
    println!("Day 1, part 2 answer: {part_2_answer}");
}
