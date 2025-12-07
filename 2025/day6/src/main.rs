use std::fs;

fn transpose_matrix<T: Copy>(input_matrix : &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = input_matrix.len();
    let cols = input_matrix[0].len();

    let mut retval = vec![Vec::with_capacity(rows); cols];
    for r in 0..rows {
        for c in 0..cols {
            retval[c].push(input_matrix[r][c]);
        }
    }
    retval
}

fn sum_vector(input : &Vec<usize>) -> usize {
    input.iter().sum()
}

fn multiply_vector(input : &Vec<usize>) -> usize {
    input.iter().product()
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let line_count = contents.lines().count();
    let mut number_matrix : Vec<Vec<usize>> = Vec::new();
    let mut line_it = contents.lines();
    for _ in 0..line_count - 1 {
        let line = line_it.next().unwrap();
        let numbers = line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        number_matrix.push(numbers);
    }
    number_matrix = transpose_matrix(&number_matrix);

    let operation_line = line_it.next().unwrap();

    let mut part_1 = 0;
    for (pos, operation) in operation_line.split_whitespace().enumerate() {
        if operation == "+" {
            part_1 += sum_vector(&number_matrix[pos]);
        } else {
            part_1 += multiply_vector(&number_matrix[pos]);
        }
    }

    println!("Day 6, part 1: {part_1}");
}
