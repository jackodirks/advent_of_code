use std::fs;
use std::cmp;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Range {
    pub range_low: usize,
    pub range_high: usize
}

impl Range {
    pub fn new(range: &str) -> Self {
        let mut range_parts = range.split("-");
        let range_low = range_parts.next().unwrap().parse().unwrap();
        let range_high = range_parts.next().unwrap().parse().unwrap();
        assert!(range_high >= range_low);
        Self{range_low, range_high}
    }

    pub fn value_in_range(&self, value: usize) -> bool {
        value >= self.range_low && value <= self.range_high
    }

    pub fn count_elements_in_range(&self) -> usize {
        self.range_high - self.range_low + 1
    }
}

fn id_is_in_some_range(ingredient_id: usize, fresh_ranges: &Vec<Range>) -> bool {
    for range in fresh_ranges {
        if range.value_in_range(ingredient_id) {
            return true;
        }
    }
    return false;
}

fn reduce_range_set(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|i| i.range_low);
    let mut cur_range = ranges[0];
    let mut retval = Vec::new();
    for range in ranges {
        if cur_range.value_in_range(range.range_low) {
            cur_range.range_high = cmp::max(cur_range.range_high, range.range_high);
        } else {
            retval.push(cur_range);
            cur_range = range;
        }
    }
    retval.push(cur_range);
    return retval;
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut content_parts = contents.split("\n\n");

    let fresh_id_ranges = content_parts.next().unwrap();
    let mut range_vec = fresh_id_ranges.lines().map(|line| Range::new(line)).collect();
    range_vec = reduce_range_set(range_vec);

    let ingredient_ids = content_parts.next().unwrap();
    let mut fresh_ingredients = 0;
    for ingredient_id in ingredient_ids.lines() {
        if id_is_in_some_range(ingredient_id.parse().unwrap(), &range_vec) {
            fresh_ingredients += 1;
        }
    }
    println!("Day 5, part 1: {fresh_ingredients}");

    let fresh_id_count = range_vec.iter().map(|range| range.count_elements_in_range()).sum::<usize>();

    println!("Day 5, part 2: {fresh_id_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_range_object() {
        let input = "3-5";
        let range = Range::new(input);
        for i in 3..6 {
            assert_eq!(range.value_in_range(i), true);
        }
        assert_eq!(range.value_in_range(6), false);
    }

    #[test]
    fn check_reduce_range_set() {
        let input = vec![
            Range::new("3-5"),
            Range::new("10-14"),
            Range::new("16-20"),
            Range::new("12-18")
        ];
        let expect = vec![
            Range::new("3-5"),
            Range::new("10-20"),
        ];
        let output = reduce_range_set(input);
        assert_eq!(output, expect);

    }

    #[test]
    fn check_reduce_range_set_1_1() {
        let range = Range::new("1-1");
        assert_eq!(range.count_elements_in_range(), 1);
    }

    #[test]
    fn check_reduce_range_set_1_2() {
        let range = Range::new("1-2");
        assert_eq!(range.count_elements_in_range(), 2);
    }

    #[test]
    fn check_reduce_range_set_3_5() {
        let range = Range::new("3-5");
        assert_eq!(range.count_elements_in_range(), 3);
    }
}
