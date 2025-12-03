pub fn count_doubles_in_range(low : u64, high : u64) -> u64 {
    let mut doubles_in_range_added_up: u64 = 0;
    for t in low..high + 1 {
        let digit_count: u32 = t.ilog10() + 1;
        if digit_count % 2 != 0 {
            continue;
        }
        let operation_digit_count = digit_count / 2;
        let operation_base: u64 = 10_u64.pow(operation_digit_count);

        let mod_val = t % operation_base;
        let div_val = t / operation_base;

        //println!("{t}, {digit_count}, {operation_digit_count} {operation_base}, {mod_val}, {div_val}");

        if mod_val == div_val {
            doubles_in_range_added_up += t
        }
    }
    return doubles_in_range_added_up;
}

pub fn number_has_repeating_groups(input: u64) -> bool {
    let digit_count = input.ilog10() + 1;
    let largest_group_to_check = digit_count / 2;

    for group_size in (1..largest_group_to_check + 1).rev() {
        if digit_count % group_size != 0 {
            continue;
        }
        let operation_base: u64 = 10_u64.pow(group_size);
        let reference_element = input % operation_base;
        let mut match_count = 1;
        //println!("{group_size}, {operation_base}, {reference_element}");
        for additional_shift in 1..digit_count/group_size {
            let tmp_operation_base : u64 = 10_u64.pow(group_size*additional_shift);
            let tmp = (input / tmp_operation_base) % operation_base;
            //println!("  {additional_shift}, {reference_element}, {tmp}");
            if tmp == reference_element {
                match_count += 1
            } else {
                break;
            }
        }
        if match_count == digit_count/group_size {
            return true;
        }
    }
    return false;
}

pub fn add_numbers_with_repeating_group_in_range(low: u64, high: u64) -> u64 {
    let mut retval: u64 = 0;
    for t in low..high + 1 {
        if number_has_repeating_groups(t) {
            retval += t;
        }
    }
    return retval;
}
