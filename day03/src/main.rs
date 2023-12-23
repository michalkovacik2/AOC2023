use core::num;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_matrix(file_name: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut line_length = 0;

    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let line_trimmed = ".".to_owned() + line.trim() + ".";
                if matrix.is_empty() {
                    line_length = line_trimmed.len();
                    matrix.push(vec!['.'; line_length]);
                }
                matrix.push(line_trimmed.chars().collect());
            }
        }
    }

    matrix.push(vec!['.'; line_length]);
    matrix
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    // RUST Closure v podstate to je lambda, ale capture je cely blok, cize ako v JS mam pocit
    matrix.into_iter().for_each(|x| {
        println!("{:?}", x);
    });
}

fn get_adjacent_numbers(matrix: & mut Vec<Vec<char>>, i: usize, j: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let directions :[[i32; 2]; 8] = [
        [-1,-1],
        [-1, 0],
        [-1, 1],
        [0 ,-1],
        [0 , 1],
        [1 ,-1],
        [1 , 0],
        [1 , 1]
    ];

    for direction in directions {
        let idx_i = (TryInto::<i32>::try_into(i).unwrap() + direction[0]) as usize;
        let idx_j = (TryInto::<i32>::try_into(j).unwrap() + direction[1]) as usize;

        if matrix[idx_i][idx_j].is_ascii_digit() {
            // Get start of number
            let mut start_of_num = 1;
            while matrix[idx_i][idx_j - start_of_num].is_ascii_digit() {
                start_of_num += 1;
            }
            // Read number
            start_of_num -= 1;
            let mut number_str = String::new();
            let mut number_idx = 0;
            while matrix[idx_i][idx_j - start_of_num + number_idx].is_ascii_digit() {
                number_str.push(matrix[idx_i][idx_j - start_of_num + number_idx]);
                matrix[idx_i][idx_j - start_of_num + number_idx] = '.';
                number_idx += 1;
            }

            vec.push(i32::from_str_radix(&number_str, 10).unwrap());
        }
    }

    vec
}

fn part1_and_part2() -> io::Result<()> {
    let mut matrix = read_matrix("input.txt");
    let mut total_sum = 0;
    let mut total_gear_ratio = 0;
    // print_matrix(&matrix);

    for i in 1 .. matrix.len() - 1 {
        for j in 1 .. matrix[0].len() - 1 {
            let c = matrix[i][j];
            if c != '.' && !c.is_ascii_digit() {
                let numbers = get_adjacent_numbers(&mut matrix, i, j);
                if c == '*' && numbers.len() == 2 {
                    total_gear_ratio += numbers.iter().product::<i32>();
                }
                total_sum += numbers.iter().sum::<i32>();
            }
        }
    }

    println!("Sum of al the part numbers: {}", total_sum);
    println!("Sum of all gear ratios:     {}", total_gear_ratio);

    Ok(())
}

fn main() {
    let _ = part1_and_part2();
}
