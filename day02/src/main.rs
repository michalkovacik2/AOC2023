use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::cmp;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum EColors {
    Red,
    Blue,
    Green
}

#[derive(Debug)]
struct Cubes {
    color_map : HashMap<EColors, u32>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Cubes>,
}

fn parse_line(line: &String) -> Game {
    let colors_map = HashMap::from([
        ("red", EColors::Red),
        ("blue", EColors::Blue),
        ("green", EColors::Green),
    ]);

    let splitted: Vec<&str> = line.split(": ").collect();
    let id_split: Vec<&str> = splitted[0].split(" ").collect();
    let id = u32::from_str_radix(id_split[1], 10).unwrap();
    let mut game = Game {
        id: id, 
        cubes: Vec::new(),
    };

    let sets_splitted: Vec<&str> = splitted[1].split(";").collect();
    for set in sets_splitted {
        let cube_str: Vec<&str> = set.split(",").collect();
        let mut color_counts_map: HashMap<EColors, u32> = HashMap::new();
        for cube in cube_str {
            let trimmed = cube.trim();
            let trimmed_split : Vec<&str> = trimmed.split(" ").collect();
            let number_of_cubes = u32::from_str_radix(trimmed_split[0], 10).unwrap();
            let color = colors_map.get(trimmed_split[1].trim()).unwrap();
            color_counts_map.insert(*color, number_of_cubes);
        }
        game.cubes.push(Cubes{color_map: color_counts_map});
    }

    game
}

fn part1_and_part2(max_red: &u32, max_green : &u32, max_blue : &u32) -> io::Result<()> {
    let mut total_sum = 0;
    let mut power_set_of_cubes = 0;

    if let Ok(file_lines) = read_lines("./input.txt") {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let game = parse_line(&line);
                let mut is_possible = true;
                let mut red_num_max = 1;
                let mut green_num_max = 1;
                let mut blue_num_max = 1;
                for cube in game.cubes {
                    let red_num = cube.color_map.get(&EColors::Red).unwrap_or(&0);
                    let green_num = cube.color_map.get(&EColors::Green).unwrap_or(&0);
                    let blue_num = cube.color_map.get(&EColors::Blue).unwrap_or(&0);
                    // PART 1
                    if red_num > max_red {
                        is_possible = false;
                    } else if blue_num > max_blue {
                        is_possible = false;
                    } else if green_num > max_green {
                        is_possible = false;
                    }
                    // PART 2
                    red_num_max = cmp::max(*red_num, red_num_max);
                    green_num_max = cmp::max(*green_num, green_num_max);
                    blue_num_max = cmp::max(*blue_num, blue_num_max);
                }
                // PART 1
                if is_possible {
                    total_sum += game.id;
                }
                // PART 2
                power_set_of_cubes += red_num_max * green_num_max * blue_num_max;
            }
        }
    }
    println!("Total sum {}", total_sum);
    println!("Power of set of cubes {}", power_set_of_cubes);

    Ok(())
}

fn main() {
    let _ = part1_and_part2(&12, &13, &14);
}