#[path = "../src/dial.rs"] mod dial;

#[cfg(test)]
mod tests {
    use crate::dial::*;

    #[test]
    fn test_parse_r10() {
        let expected = 10;
        let input = "R10";
        let result = parse_dial_operation(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_r20() {
        let expected = 20;
        let input = "R20";
        let result = parse_dial_operation(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_l200() {
        let expected = -200;
        let input = "L200";
        let result = parse_dial_operation(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_r20_from_40_is_60() {
        let expected = 60;
        let operation = "R20";
        let current_value = 40;
        let result = new_dial_value(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_l200_from_0_is_0() {
        let expected = 0;
        let operation = "L200";
        let current_value = 0;
        let result = new_dial_value(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_l68_from_50_is_82() {
        let expected = 82;
        let operation = "L68";
        let current_value = 50;
        let result = new_dial_value(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_r75_from_50_has_1_zero_crossing() {
        let expected = 1;
        let operation = "R75";
        let current_value = 50;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_l175_from_50_has_2_zero_crossing() {
        let expected = 2;
        let operation = "L175";
        let current_value = 50;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_l50_from_50_has_0_zero_crossing() {
        let expected = 0;
        let operation = "L50";
        let current_value = 50;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_r48_from_52_has_0_zero_crossing() {
        let expected = 0;
        let operation = "R48";
        let current_value = 52;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_r100_from_0_has_1_zero_crossing() {
        let expected = 1;
        let operation = "R100";
        let current_value = 0;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_r250_from_50_has_2_zero_crossings() {
        let expected = 2;
        let operation = "R250";
        let current_value = 50;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_l50_from_50_has_0_zero_crossings() {
        let expected = 0;
        let operation = "L50";
        let current_value = 50;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_l128_from_28_has_1_zero_crossing() {
        let expected = 1;
        let operation = "L128";
        let current_value = 28;
        let result = count_zero_crossings(current_value, operation);
        assert_eq!(expected, result);
    }
}
