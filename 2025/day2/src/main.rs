mod product_id_range;

use std::fs;
use product_id_range::*;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_doubles = 0;
    let mut total_any_repeat = 0;
    let parts = contents.trim().split(",");
    for part in parts {
        if part.is_empty() {
            continue;
        }
        let mut range = part.split("-");
        let range_low: u64 = range.next().unwrap().parse().unwrap();
        let range_high: u64 = range.next().unwrap().parse().unwrap();
        let doubles = count_doubles_in_range(range_low, range_high);
        total_doubles += doubles;

        total_any_repeat += add_numbers_with_repeating_group_in_range(range_low, range_high);
    }
    println!("Day 2, part 1: {total_doubles}");
    println!("Day 2, part 2: {total_any_repeat}");
}
