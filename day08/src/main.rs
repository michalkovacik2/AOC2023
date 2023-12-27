use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const MAX_STARTS_AND_ENDS: i32 = 10;

#[derive(Debug, PartialEq, Clone, Copy)]
enum EInstruction {
    Left,
    Right,
}

#[derive(Debug, Default)]
struct Network {
    instructions : Vec<EInstruction>,
    nodes: Vec<(i32, i32)>,
}

fn insert_or_get(mapping: &mut HashMap<String, i32>, indexes: &mut [i32; 3], s: &String, network: &mut Network) -> i32 {
    let ret_val: i32;
    let mapping_to_val = mapping.get(s);
    if mapping_to_val.is_none() {
        if s.ends_with("A") {
            mapping.insert(s.to_string(), indexes[0]);
            ret_val = indexes[0];
            indexes[0] += 1;
        } else if s.ends_with("Z") {
            mapping.insert(s.to_string(), indexes[1]);
            ret_val = indexes[1];
            indexes[1] += 1;
        } else {
            mapping.insert(s.to_string(), indexes[2]);
            ret_val = indexes[2];
            indexes[2] += 1;
            network.nodes.push((-1, -1));
        }
    } else {
        ret_val = *mapping_to_val.unwrap();
    }
    ret_val
}

fn parse_input(file_name: &str) -> Network {
    let mut network = Network::default();
    let mut file_lines = read_lines(file_name).unwrap();
    let instructions_str = file_lines.next().unwrap().unwrap();
    instructions_str.trim().chars().for_each(|x| {
         network.instructions.push(if x == 'L' {EInstruction::Left} else {EInstruction::Right});
    });
    file_lines.next(); // Eat empty line

    let mut mapping: HashMap<String, i32> = HashMap::from([
        ("AAA".to_owned(), 0),
        ("ZZZ".to_owned(), MAX_STARTS_AND_ENDS),
    ]);
    let mut indexes = [1, MAX_STARTS_AND_ENDS + 1, 2 * MAX_STARTS_AND_ENDS];
    for _ in 0..MAX_STARTS_AND_ENDS * 2 {
        network.nodes.push((-1, -1));
    }

    for file_line in file_lines {
        if let Ok(line) = file_line {
            let splitted: Vec<&str> = line.split(" ").collect();
            let from_str = splitted[0].to_owned();
            let left_str = splitted[2][1..splitted[2].len()-1].to_owned();
            let right_str = splitted[3][0..splitted[3].len()-1].to_owned();
            
            let left = insert_or_get(&mut mapping, &mut indexes, &left_str, &mut network);
            let right = insert_or_get(&mut mapping, &mut indexes, &right_str, &mut network);
            let from = insert_or_get(&mut mapping, &mut indexes, &from_str, &mut network);
            network.nodes[from as usize] = (left, right);
        }
    }

    network
}

fn is_at_end(node: i32) -> bool {
    node >= MAX_STARTS_AND_ENDS && node < 2 * MAX_STARTS_AND_ENDS
}

fn get_number_of_steps(network: &Network, start_node: i32) -> i64 {
    let mut current_node = start_node;
    let mut current_ins_idx = 0;
    let mut number_of_steps: i64 = 0;
    while !is_at_end(current_node) {
        if network.instructions[current_ins_idx] == EInstruction::Left {
            current_node = network.nodes[current_node as usize].0;
        } else {
            current_node = network.nodes[current_node as usize].1;
        }
        current_ins_idx = if current_ins_idx + 1 >= network.instructions.len() {0} else {current_ins_idx + 1};
        number_of_steps += 1;
    }
    number_of_steps
}

fn part1() -> io::Result<()> {
    let mut network = parse_input("input.txt");
    for i in 0..2 * MAX_STARTS_AND_ENDS {
        if i == 0 || i == MAX_STARTS_AND_ENDS {
            continue;
        }
        network.nodes[i as usize] = (-1, -1);
    }
    println!("Number of steps: {}", get_number_of_steps(&network, 0));
    Ok(())
}

fn gcd(number1: i64, number2: i64) -> i64 {
    let mut r; 
    let mut a = number1;
    let mut b = number2;
    while a % b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    b
}

fn lcm(number1: i64, number2: i64) -> i64 {
    number1 * number2 / gcd(number1, number2)
}

fn part2() -> io::Result<()> {
    let network = parse_input("input.txt");
    let mut number_of_steps_to_end: Vec<i64> = Vec::new();
    for i in 0..MAX_STARTS_AND_ENDS {
        if network.nodes[i as usize].0 != -1 && network.nodes[i as usize].1 != -1 {
            number_of_steps_to_end.push(get_number_of_steps(&network, i as i32));
        }
    }

    let mut result = lcm(number_of_steps_to_end[0], number_of_steps_to_end[1]);
    for i in 2..number_of_steps_to_end.len() {
        result = lcm(result, number_of_steps_to_end[i]);
    }

    println!("Number of steps: {}", result);
    Ok(())
}

fn main() {
    // let _ = part1();
    let _ = part2();
}
