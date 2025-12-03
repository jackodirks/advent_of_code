pub fn parse_dial_operation(operation: &str) -> i32 {
    let mut inverter: i32 = 1;
    if &operation[0..1] == "L" {
        inverter = -1
    }
    if inverter == 1 {
        assert!(&operation[0..1] == "R");
    }
    let val: i32 = (&operation[1..]).parse().unwrap();
    return val*inverter
}

pub fn new_dial_value(current_value: i32, operation: &str) -> i32 {
    (current_value + parse_dial_operation(operation)).rem_euclid(100)
}

pub fn count_zero_crossings(current_value: i32, operation: &str) -> i32 {
    assert!(current_value >= 0);
    let result_value = current_value + parse_dial_operation(operation);
    let zero_cross_count = (result_value/100).abs();
    if result_value < 0 {
        if current_value > 0 && result_value % 100 != 0 {
            return zero_cross_count + 1;
        }
        return zero_cross_count;
    }

    if result_value % 100 == 0 && current_value > 0 && zero_cross_count > 0 {
        return zero_cross_count - 1;
    }
    return zero_cross_count;
}
