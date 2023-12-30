use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::cmp;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Node {
    x: usize,
    y: usize,
    cost: i64,
}

// Returns (Matrix, Position of the animal)
fn read_matrix(file_name: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut animal_pos = (0, 0);
    let mut line_length = 0;

    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let line_trimmed = ".".to_owned() + line.trim() + ".";
                if matrix.is_empty() {
                    line_length = line_trimmed.len();
                    matrix.push(vec!['.'; line_length]);
                }
                if let Some(pos) = line.chars().position(|x| x == 'S') {
                    animal_pos = (matrix.len(), pos + 1);
                }
                matrix.push(line_trimmed.chars().collect());
            }
        }
    }

    matrix.push(vec!['.'; line_length]);
    (matrix, animal_pos)
}

// Returns (Matrix where each character is expanded to 3x3, Position of the animal)
fn read_matrix_v2(file_name: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut animal_pos = (0, 0);
    let mut line_length = 0;

    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                line_length = (line.trim().chars().count() + 2) * 3;
                if matrix.is_empty() {
                    matrix.push(vec!['.'; line_length]);
                    matrix.push(vec!['.'; line_length]);
                    matrix.push(vec!['.'; line_length]);
                }
                
                let mut line1: Vec<char> = Vec::new();
                let mut line2: Vec<char> = Vec::new();
                let mut line3: Vec<char> = Vec::new();

                line1.push('.'); line1.push('.'); line1.push('.');
                line2.push('.'); line2.push('.'); line2.push('.');
                line3.push('.'); line3.push('.'); line3.push('.');

                for (pos, c) in line.trim().chars().enumerate() {
                    match c {
                        '.' => {
                            line1.push('.'); line1.push('.'); line1.push('.');
                            line2.push('.'); line2.push('.'); line2.push('.');
                            line3.push('.'); line3.push('.'); line3.push('.');
                        }
                        '|' => {
                            line1.push('.'); line1.push('X'); line1.push('.');
                            line2.push('.'); line2.push('X'); line2.push('.');
                            line3.push('.'); line3.push('X'); line3.push('.');
                        }
                        '-' => {
                            line1.push('.'); line1.push('.'); line1.push('.');
                            line2.push('X'); line2.push('X'); line2.push('X');
                            line3.push('.'); line3.push('.'); line3.push('.');
                        }
                        'L' => {
                            line1.push('.'); line1.push('X'); line1.push('.');
                            line2.push('.'); line2.push('X'); line2.push('X');
                            line3.push('.'); line3.push('.'); line3.push('.');
                        }
                        'J' => {
                            line1.push('.'); line1.push('X'); line1.push('.');
                            line2.push('X'); line2.push('X'); line2.push('.');
                            line3.push('.'); line3.push('.'); line3.push('.');
                        }
                        '7' => {
                            line1.push('.'); line1.push('.'); line1.push('.');
                            line2.push('X'); line2.push('X'); line2.push('.');
                            line3.push('.'); line3.push('X'); line3.push('.');
                        }
                        'F' => {
                            line1.push('.'); line1.push('.'); line1.push('.');
                            line2.push('.'); line2.push('X'); line2.push('X');
                            line3.push('.'); line3.push('X'); line3.push('.');
                        }
                        'S' => {
                            animal_pos = (matrix.len(), (pos + 1) * 3);
                            line1.push('.'); line1.push('.'); line1.push('.');
                            line2.push('.'); line2.push('.'); line2.push('.');
                            line3.push('.'); line3.push('.'); line3.push('.');
                        }
                        _ => {
                            panic!("Should not happen");
                        }
                    }
                }
                
                line1.push('.'); line1.push('.'); line1.push('.');
                line2.push('.'); line2.push('.'); line2.push('.');
                line3.push('.'); line3.push('.'); line3.push('.');

                matrix.push(line1);
                matrix.push(line2);
                matrix.push(line3);
            }
        }
    }

    matrix.push(vec!['.'; line_length]);
    matrix.push(vec!['.'; line_length]);
    matrix.push(vec!['.'; line_length]);
    (matrix, animal_pos)
}

fn replace_animal_position_with_correct_path(matrix: &mut Vec<Vec<char>>, animal_position: (usize, usize)) {
    let (mut up, mut down, mut left, mut right) = (false, false, false, false);
    let (x, y) = (animal_position.0, animal_position.1);
    if matrix[x - 1][y] == '|' || matrix[x - 1][y] == '7' || matrix[x - 1][y] == 'F' {
        up = true;
    }

    if matrix[x + 1][y] == '|' || matrix[x + 1][y] == 'L' || matrix[x + 1][y] == 'J' {
        down = true;
    }

    if matrix[x][y - 1] == '-' || matrix[x][y - 1] == 'L' || matrix[x][y - 1] == 'F' {
        left = true;
    }

    if matrix[x][y + 1] == '-' ||  matrix[x][y + 1] == '7' || matrix[x][y + 1] == 'J' {
        right = true;
    }

    // It is a loop so I must have two bools set
    if up && down    { matrix[x][y] = '|' }
    if left && right { matrix[x][y] = '-' }
    
    if up && left    { matrix[x][y] = 'J' }
    if up && right   { matrix[x][y] = 'L' }
    
    if down && left  { matrix[x][y] = '7' }
    if down && right { matrix[x][y] = 'F' }
}

fn replace_animal_position_with_correct_path_v2(matrix: &mut Vec<Vec<char>>, animal_position: (usize, usize)) {
    matrix[animal_position.0 + 1][animal_position.1 + 1] = 'X';
    if matrix[animal_position.0 - 1][animal_position.1 + 1] == 'X' {
        matrix[animal_position.0][animal_position.1 + 1] = 'X';
    }
    if matrix[animal_position.0 + 1][animal_position.1 - 1] == 'X' {
        matrix[animal_position.0 + 1][animal_position.1] = 'X';
    }
    if matrix[animal_position.0 + 3][animal_position.1 + 1] == 'X' {
        matrix[animal_position.0 + 2][animal_position.1 + 1] = 'X';
    }
    if matrix[animal_position.0 + 1][animal_position.1 + 3] == 'X' {
        matrix[animal_position.0 + 1][animal_position.1 + 2] = 'X';
    }
}

fn bfs(matrix: &Vec<Vec<char>>, distance_matrix: &mut Vec<Vec<i64>>, animal_position: (usize, usize)) -> i64 {
    let mut maximum_cost: i64 = 0;
    let mut queue: VecDeque<Node> = VecDeque::new();
    let directions_map: HashMap<char, [(i32, i32); 2]> = HashMap::from([
        ('|', [(-1,  0), (1 ,  0)]),
        ('-', [(0 , -1), (0 ,  1)]),
        ('L', [(-1,  0), (0 ,  1)]),
        ('J', [(-1,  0), (0 , -1)]),
        ('7', [(1 ,  0), (0 , -1)]),
        ('F', [(1 ,  0), (0 ,  1)]),
    ]);
    
    queue.push_back( Node {
        x: animal_position.0, 
        y: animal_position.1, 
        cost: 0
    });
        
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let current_path_char = matrix[node.x][node.y];
        if let Some(directions) = directions_map.get(&current_path_char) {
            for direction in directions {
                let new_x = (node.x as i32 + direction.0) as usize;
                let new_y = (node.y as i32 + direction.1) as usize;
                if matrix[new_x][new_y] != '.' && distance_matrix[new_x][new_y] == -1 {
                    distance_matrix[new_x][new_y] = node.cost + 1;
                    queue.push_back( Node {
                        x: new_x, 
                        y: new_y, 
                        cost: node.cost + 1
                    });
                    maximum_cost = cmp::max(maximum_cost, node.cost + 1);
                }
            }
        }
    }

    maximum_cost
}

// NOTE: Recursive implementation overflows stack, so I must use iterative implementation
fn dfs(matrix: &mut Vec<Vec<char>>, distance_matrix: &mut Vec<Vec<i64>>, steps: &[[i32; 2]; 4], node: Node, mark: i64, following: char) {
    let mut stack: Vec<Node> = Vec::new(); 
    stack.push(node);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        distance_matrix[node.x][node.y] = mark;
        for step in steps {
            let x = node.x as i32 + step[0];
            let y = node.y as i32 + step[1];
            if x >= 0 && x <= (distance_matrix.len() - 1) as i32 &&
               y >= 0 && y <= (distance_matrix[0].len() - 1) as i32 &&
               distance_matrix[x as usize][y as usize] == -1 &&
               matrix[x as usize][y as usize] == following {
                    stack.push(Node{x: x as usize, y: y as usize, cost: 0});
               }
        }
    }
}

fn print_distance_matrix(distance_matrix: &Vec<Vec<i64>>) {
    for row in distance_matrix {
        for x in row {
            if *x == -1 {
                print!(".. ")
            } else {
                print!("{} ", format!("{:0>2}", x));
            }
        }
        println!("");
    }
}

fn part1(file_name: &str) -> io::Result<()> {
    let (mut matrix, animal_position) = read_matrix(file_name);
    let mut distance_matrix = vec![vec![-1; matrix[0].len()]; matrix.len()];

    replace_animal_position_with_correct_path(&mut matrix, animal_position);
    distance_matrix[animal_position.0][animal_position.1] = 0;
    let max_cost = bfs(&matrix, &mut distance_matrix, animal_position);
    
    println!("Maximum number of steps: {}", max_cost);
    Ok(())
}

fn part2(file_name: &str) -> io::Result<()> {
    let mut num_tiles_enclosed_by_loop = 0;
    let (mut matrix, animal_position) = read_matrix_v2(file_name);
    let mut distance_matrix = vec![vec![-1; matrix[0].len()]; matrix.len()];

    replace_animal_position_with_correct_path_v2(&mut matrix, animal_position);
    distance_matrix[animal_position.0 + 1][animal_position.1 + 1] = 0;
    let steps = [ [-1, 0], [0, -1], [1, 0], [0, 1] ];
    // 1. Mark the closed loop where animal is with 1
    dfs(&mut matrix, &mut distance_matrix, &steps, Node{x: animal_position.0 + 1, y: animal_position.1 + 1, cost: 0}, 1, 'X');
    // 2. Flood fill everything from outside with 0
    dfs(&mut matrix, &mut distance_matrix, &steps, Node{x: 0, y: 0, cost: 0}, 0, '.');
    // 3. What remained is the inside of our loop
    for i in (0..distance_matrix.len()).step_by(3) {
        for j in (0..distance_matrix[0].len()).step_by(3) {
            let mut all_3x3_not_visited = true;
            for k in i..i + 3 {
                for l in j..j + 3 {
                    if distance_matrix[k][l] != -1 {
                        all_3x3_not_visited = false;
                        break;
                    }
                }
                if !all_3x3_not_visited {
                    break;
                }
            }
            if all_3x3_not_visited {
                num_tiles_enclosed_by_loop += 1;
            }
        }
    }

    println!("Number of tiles enclosed by the loop: {}", num_tiles_enclosed_by_loop);
    Ok(())
}

fn main() {
    // let _ = part1("input.txt");
    let _ = part2("input.txt");
}