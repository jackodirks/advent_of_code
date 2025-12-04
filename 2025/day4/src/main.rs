use std::fs;
use std::cmp;

fn toiletroll_matrix_from_string(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars()
            .map(|c| match c {
                '.' => 0,
                '@' => 1,
                _ => panic!("Unexpected character: {c}"),
            })
            .collect())
        .collect()
}

fn heat_map_from_toiletroll_matrix(input: &[Vec<u8>])-> Vec<Vec<u8>> {
    let mut heat_map = vec![vec![0; input[0].len()]; input.len()];
    let min_x = 0;
    let max_x = input.len() - 1;
    let min_y = 0;
    let max_y = input[0].len() - 1;
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let start_x = cmp::max(min_x, x.saturating_sub(1));
            let end_x = cmp::min(max_x, x + 1);
            let start_y = cmp::max(min_y, y.saturating_sub(1));
            let end_y = cmp::min(max_y, y + 1);
            let val = input[x][y];
            for x_index in start_x..end_x + 1 {
                for y_index in start_y..end_y + 1 {
                    if x_index == x && y_index == y {
                        continue;
                    }
                    heat_map[x_index][y_index] += val;
                }
            }
        }
    }
    return heat_map;
}

fn count_accessable_rolls_of_paper(toiletroll_matrix: &[Vec<u8>], heat_map: &[Vec<u8>]) -> u32 {
    let min_x = 0;
    let max_x = toiletroll_matrix.len() - 1;
    let min_y = 0;
    let max_y = toiletroll_matrix[0].len() - 1;
    let mut retval = 0;
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if heat_map[x][y] < 4 && toiletroll_matrix[x][y] == 1 {
                retval += 1;
            }
        }
    }
    return retval;
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let toiletroll_matrix = toiletroll_matrix_from_string(&contents);
    let heat_map = heat_map_from_toiletroll_matrix(&toiletroll_matrix);
    let accessable_rolls = count_accessable_rolls_of_paper(&toiletroll_matrix, &heat_map);
    println!("Day 4, part 1: {accessable_rolls}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_to_toiletroll_matrix() {
        let input = "..@@.\n@@...";
        let expect = vec![
            vec![0,0,1,1,0],
            vec![1,1,0,0,0],
        ];
        let output = toiletroll_matrix_from_string(input);
        assert_eq!(output, expect)
    }

    #[test]
    fn check_heatmap_from_toiletroll_matrix() {
        let input = vec![
            vec![1,0,1,0,1],
            vec![0,1,1,1,0],
            vec![1,1,0,0,0],
        ];
        let expect = vec![
            vec![1,4,3,4,1],
            vec![4,5,4,3,2],
            vec![2,3,4,2,1],
        ];
        let output = heat_map_from_toiletroll_matrix(&input);
        assert_eq!(output, expect)

    }

    #[test]
    fn check_accessable_rolls_of_paper() {
        let toiletroll_matrix = vec![
            vec![1,0,1,0,1],
            vec![0,1,1,1,0],
            vec![1,1,0,0,0],
        ];
        let heat_map = vec![
            vec![1,4,3,4,1],
            vec![4,5,4,3,2],
            vec![2,3,4,2,1],
        ];
        let expect = 6;
        let output = count_accessable_rolls_of_paper(&toiletroll_matrix, &heat_map);
        assert_eq!(output, expect)

    }

}
