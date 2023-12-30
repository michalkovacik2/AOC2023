use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Node {
    x: i64,
    y: i64,
    x_expanded: i64,
    y_expanded: i64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_matrix(file_name: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let line_trimmed = line.trim();
                matrix.push(line_trimmed.chars().collect());
                // if line_trimmed.chars().all(|x| x == '.') {
                //     matrix.push(line_trimmed.chars().collect());
                // }
            }
        }
    }

    matrix
}

// fn expand_matrix_columns(matrix: &mut Vec<Vec<char>>) {
//     let mut columns_for_expansion: Vec<usize> = Vec::new();

//     for i in 0..matrix[0].len() {
//         let mut all_dots = true;
//         for j in 0..matrix.len() {
//             if matrix[j][i] != '.' {
//                 all_dots = false;
//                 break;
//             }
//         }
//         if all_dots {
//             columns_for_expansion.push(i);
//         }
//     }

//     for i in 0..columns_for_expansion.len() {
//         for j in 0..matrix.len() {
//             matrix[j].insert(columns_for_expansion[i] + i, '.');
//         }
//     }
// }

fn get_nodes(matrix: &Vec<Vec<char>>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '#' {
                nodes.push(Node{ x: i as i64, y: j as i64, x_expanded: i as i64, y_expanded: j as i64});
            }
        }
    }

    nodes
}

fn calculate_expansion(matrix: &Vec<Vec<char>>, nodes: &mut Vec<Node>, expansion_step: i64) {
    // rows
    for i in 0..matrix.len() {
        if matrix[i].iter().all(|x| *x == '.') {
            for node in nodes.iter_mut() {
                if node.x > i as i64 {
                    node.x_expanded += expansion_step - 1;
                }
            }
        }
    }

    // cols
    for i in 0..matrix[0].len() {
        let mut all_dots = true;
        for j in 0..matrix.len() {
            if matrix[j][i] != '.' {
                all_dots = false;
                break;
            }
        }
        if all_dots {
            for node in nodes.iter_mut() {
                if node.y > i as i64 {
                    node.y_expanded += expansion_step - 1;
                }
            }
        }
    }
}

fn part1_and_part2(file_name: &str, expansion_step: i64) -> io::Result<()> {
    let matrix = read_matrix(file_name);
    let mut nodes = get_nodes(&matrix);
    calculate_expansion(&matrix, &mut nodes, expansion_step);

    let mut sum = 0;
    for i in 0..nodes.len() {
        for j in i..nodes.len() {
            sum +=  i64::abs(nodes[j].x_expanded - nodes[i].x_expanded) + i64::abs(nodes[j].y_expanded - nodes[i].y_expanded);
        }
    }

    println!("Sum of the lengths: {}", sum);
    Ok(())
}

fn main() {
    let _ = part1_and_part2("input.txt", 1000000);
}
