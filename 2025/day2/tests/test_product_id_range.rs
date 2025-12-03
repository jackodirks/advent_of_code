#[path = "../src/product_id_range.rs"] mod product_id_range;

#[cfg(test)]
mod tests {
    use crate::product_id_range::*;

    #[test]
    fn test_doubles_in_11_22_add_to_55() {
        let expected = 33;
        let low = 11;
        let high = 22;
        let result = count_doubles_in_range(low, high);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_doubles_in_1188511880_1188511890_add_to_1188511885() {
        let expected = 1188511885;
        let low = 1188511880;
        let high = 1188511890;
        let result = count_doubles_in_range(low, high);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_doubles_in_99_115_add_to_99() {
        let expected = 99;
        let low = 99;
        let high = 115;
        let result = count_doubles_in_range(low, high);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_12345678_has_no_repeating_groups() {
        let expected = false;
        let input = 12345678;
        let result = number_has_repeating_groups(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_1111111_has_repeating_groups() {
        let expected = true;
        let input = 1111111;
        let result = number_has_repeating_groups(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_2121212121_has_repeating_groups() {
        let expected = true;
        let input = 2121212121;
        let result = number_has_repeating_groups(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_2121212122_has_no_repeating_groups() {
        let expected = false;
        let input = 2121212122;
        let result = number_has_repeating_groups(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_2122222222_has_no_repeating_groups() {
        let expected = false;
        let input = 2122222222;
        let result = number_has_repeating_groups(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_add_numbers_with_repeating_group_in_range_998_1012() {
        let expected = 999+1010;
        let low = 998;
        let high = 1012;
        let result = add_numbers_with_repeating_group_in_range(low, high);
        assert_eq!(expected, result);
    }
}
