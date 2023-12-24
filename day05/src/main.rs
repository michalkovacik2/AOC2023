use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::cmp;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Default, Debug)]
struct RangeMap {
    source_start: i64,
    source_end: i64,
    destination_start: i64,
    destination_end: i64, 
}

fn parse_input(file_name: &str) -> (Vec<i64>, Vec<Vec<RangeMap>>) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Vec<RangeMap>> = Vec::new();

    if let Ok(mut file_lines) = read_lines(file_name) {
        while let Some(file_line) = file_lines.next() {
            if let Ok(line) = file_line {
                let splitted: Vec<&str> = line.split(":").collect();
                if splitted[0] == "seeds" {
                    // Seeds
                    let seeds_str: Vec<&str> = splitted[1].trim().split(" ").collect();
                    for seed in seeds_str {
                        seeds.push(i64::from_str_radix(seed, 10).unwrap());
                    }
                    // Eat empty line after seeds :)
                    file_lines.next();
                } else {
                    // Maps
                    maps.push(Vec::new());
                    while let Some(next_line_file) = file_lines.next() {
                        if let Ok(next_line) = next_line_file {
                            // Break when empty line
                            if next_line.trim().is_empty() {
                                break;
                            }

                            let range_splitted: Vec<&str> = next_line.trim().split(" ").collect();
                            // (destination start) (source start) (how much)
                            let destination_start = i64::from_str_radix(range_splitted[0], 10).unwrap();
                            let source_start = i64::from_str_radix(range_splitted[1], 10).unwrap();
                            let range_length = i64::from_str_radix(range_splitted[2], 10).unwrap();
                            let last_element_idx = maps.len() - 1;
                            let mut index_where_should_range_go = 0;
                            while index_where_should_range_go < maps[last_element_idx].len() 
                                  && source_start > maps[last_element_idx][index_where_should_range_go].source_start {
                                index_where_should_range_go += 1;
                            }
                            
                            maps[last_element_idx].insert(index_where_should_range_go, RangeMap {
                                source_start: source_start,
                                source_end: source_start + range_length - 1,
                                destination_start: destination_start,
                                destination_end: destination_start + range_length - 1 
                            });
                        }
                    }
                }
            }
        }
    }

    (seeds, maps)
}

fn part1() -> io::Result<()> {
    let mut minimum_location_number = i64::MAX;
    let (seeds, maps) = parse_input("input.txt");
    // println!("{:#?}", seeds);
    // println!("{:#?}", maps);

    for seed in seeds {
        let mut current_mapped_number = seed;
        for list_maps in maps.iter() {
            for map in list_maps {
                if current_mapped_number >= map.source_start && current_mapped_number <= map.source_end {
                    let idx_from_start = current_mapped_number - map.source_start;
                    current_mapped_number = map.destination_start + idx_from_start;
                    break;
                }
            }
        }
        minimum_location_number = cmp::min(minimum_location_number, current_mapped_number);
    }

    println!("Lowest location number: {}", minimum_location_number);

    Ok(())
}

fn part2() -> io::Result<()> {
    // Naive implementation from part1 and using for loop takes infinite time xD
    // We need something better (Probably not this monstrosity)
    // Strasny algoritmus som tu spravil (NOTE: Ono je dost mozne ze som mal len stastie a na dvoch vstupoch to dalo dobre cislo)
    let mut minimum_location_number = i64::MAX;
    let (seeds, maps) = parse_input("input.txt");

    // I am trying to make range table from input, something like this (for example input first seed range):
    //  input   | map1     | map2     | map3     | map4     | map5     | map6     | map7     |
    // (79, 92) | (81, 94) | (81, 94) | (81, 94) | (74, 87) | (78, 80) | (78, 80) | (82, 84) |
    //          |          |          |          |          | (45, 55) | (46, 56) | (46, 55) |
    //          |          |          |          |          |          |          | (56, 56) |
    // Then it is trivial look at the end (map7) at the start value and pick the lowest one (46).
    for i in (0..seeds.len()).step_by(2) {
        let mut idx_ranges = 0;
        let mut ranges: Vec<Vec<(i64, i64)>> = vec![vec![]; maps.len() + 1];
        // Range <start, end> (start is in the interval and also end is in the interval)
        ranges[idx_ranges].push((seeds[i], seeds[i] + seeds[i + 1] - 1));
        
        for all_maps in maps.iter() {
            for i in 0..ranges[idx_ranges].len() {
                let mut start = ranges[idx_ranges][i].0;
                let end = ranges[idx_ranges][i].1;
                let mut all_numbers_used = false;
                let mut j = 0;

                while j < all_maps.len() {
                    if end < all_maps[j].source_start {
                        ranges[idx_ranges + 1].push((start, end));
                        all_numbers_used = true;
                        break;
                    } else if start > all_maps[j].source_end {
                        j += 1;
                        continue;
                    } else if start < all_maps[j].source_start {
                        ranges[idx_ranges + 1].push((start, all_maps[j].source_start - 1));
                        start = all_maps[j].source_start;
                        j += 1;
                    } else if start >= all_maps[j].source_start && end <= all_maps[j].source_end {
                        let r_start = start - all_maps[j].source_start + all_maps[j].destination_start;
                        let r_end = end - all_maps[j].source_start + all_maps[j].destination_start;
                        ranges[idx_ranges + 1].push((r_start, r_end));
                        all_numbers_used = true;
                        break;
                    } else if start >= all_maps[j].source_start && end > all_maps[j].source_end {
                        let r_start = start - all_maps[j].source_start + all_maps[j].destination_start;
                        let r_end = all_maps[j].destination_end;
                        ranges[idx_ranges + 1].push((r_start, r_end));
                        start = all_maps[j].source_end + 1;
                        j += 1;
                    } else {
                        // I most probably did but my input was very generous to me :D
                        panic!("Sanity check: I hope I didn't miss some case");
                    }
                }

                if !all_numbers_used {
                    ranges[idx_ranges + 1].push((start, end));
                }
            }

            idx_ranges += 1;
        }

        ranges[ranges.len() - 1].iter().for_each(|x|{
            minimum_location_number = cmp::min(minimum_location_number, x.0);
        });
    } 

    println!("Lowest location number: {}", minimum_location_number);
    
    Ok(())
}

fn main() {
    // let _ = part1();
    let _ = part2();
}
