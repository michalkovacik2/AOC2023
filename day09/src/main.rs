use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_and_process_line(line: &String) -> Vec<Vec<i64>> {
    // Parse input
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    numbers.push(Vec::new());
    line.split_whitespace().for_each(|x| { numbers[0].push(i64::from_str_radix(x, 10).unwrap()) });
    
    // Solve
    let mut index = 0;
    loop {
        numbers.push(vec![0; numbers[index].len() - 1]);
        let mut all_zeros = true;
        for i in 1..numbers[index].len() {
            numbers[index + 1][i - 1] = numbers[index][i] - numbers[index][i - 1];
            if numbers[index + 1][i - 1] != 0 {
                all_zeros = false;
            }
        }

        if all_zeros {
            break;
        }
        index += 1;
    }

    numbers
}

fn part1(file_name: &str) -> io::Result<()> {
    /*
    10  13  16  21  30  45  E   | E = D + 45
       3   3   5   9  15  D     | D = C + 15
         0   2   4   6   C      | C = B + 6
           2   2   2   B        | B = A + 2
             0   0   A          | A = 0
    */
    let mut sum: i64 = 0;
    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let numbers = parse_and_process_line(&line);

                let mut extrapolated_value = 0;
                for i in (0..numbers.len() - 1).rev() {
                    extrapolated_value += numbers[i][numbers[i].len() - 1]
                }

                sum += extrapolated_value;
            }
        }
    }
    
    println!("Sum of extrapolated values: {}", sum);
    Ok(())
}

fn part2(file_name: &str) -> io::Result<()> {
    /*
    E 10  13  16  21  30  45 | E = 10 - D
     D  3   3   5   9   15   | D = 3 - C
       C  0   2   4   6      | C = 0 - B
         B  2   2   2        | B = 2 - A
           A  0   0          | A = 0
    */
    let mut sum: i64 = 0;
    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let numbers = parse_and_process_line(&line);

                let mut extrapolated_value = 0;
                for i in (0..numbers.len() - 1).rev() {
                    extrapolated_value = numbers[i][0] - extrapolated_value;
                }

                sum += extrapolated_value;
            }
        }
    }
    
    println!("Sum of extrapolated values: {}", sum);
    Ok(())
}

fn main() {
//    let _ = part1("input.txt");
   let _ = part2("input.txt");
}
