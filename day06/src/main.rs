use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::iter::zip;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(file_name: &str) -> (Vec<i64>, Vec<i64>) {
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    let mut file_lines = read_lines(file_name).unwrap();
    let first_line = file_lines.next().unwrap().unwrap();
    let second_line = file_lines.next().unwrap().unwrap();

    first_line.split_whitespace().for_each(|x|{
        if let Ok(num) = i64::from_str_radix(x, 10) {
            times.push(num);
        }
    });

    second_line.split_whitespace().for_each(|x|{
        if let Ok(num) = i64::from_str_radix(x, 10) {
            distances.push(num);
        }
    });

    (times, distances)
} 

/*
    Riesenie kvadratickej nerovnice (priklad pre time: 7 ms; distance: 9 mm)
    x * (7 - x)   > 9 
    -x^2 + 7x - 9 > 0
    Vypocitam korene x1 a x2 standardne cez diskriminant a vzorec.
    Mam minus pred x^2, takze parabola bude konkavna a riesenia, kde je vacsia ako nula budu medzi korenmi x1 a x2.
    Riesenim su iba prirodzene cisla (Pozn. Mame ostru nerovnost, takze ak koren je prirodzene cislo, tak ho neberiem do uvahy)
 */
fn calculate_ways_to_win(times: &Vec<i64>, distances: &Vec<i64>) -> i64 {
    let mut num_ways_to_beat_record_product = 1;
    for (time, distance) in zip(times, distances) {
        let d = time * time - 4 * -1 * -distance;
        let x1: f64 = (-(*time as f64) + ((d as f64).sqrt())) / -2.0;
        let x2: f64 = (-(*time as f64) - ((d as f64).sqrt())) / -2.0;
        let x1_int = if x1.ceil()  == x1 { x1.ceil()  as i64 + 1 } else { x1.ceil() as i64 };
        let x2_int = if x2.floor() == x2 { x2.floor() as i64 - 1 } else { x2.floor() as i64 };
        num_ways_to_beat_record_product *= x2_int - x1_int + 1;
    }
    num_ways_to_beat_record_product
}

fn part1() -> io::Result<()> {
    let (times, distances) = parse_input("example.txt");
    println!("Number of ways to beat record (product): {}", calculate_ways_to_win(&times, &distances));
    Ok(())
}

fn part2() -> io::Result<()> {
    let (mut times, mut distances) = parse_input("input.txt");
    
    let times_str: Vec<String> = times.iter().map(|x|{x.to_string()}).collect();
    let time = i64::from_str_radix(times_str.join("").as_str(), 10).unwrap();

    let distance_str: Vec<String> = distances.iter().map(|x|{x.to_string()}).collect();
    let distance = i64::from_str_radix(distance_str.join("").as_str(), 10).unwrap();

    times.clear();
    distances.clear();

    times.push(time);
    distances.push(distance);

    println!("Number of ways to beat record (product): {}", calculate_ways_to_win(&times, &distances));
    Ok(())
}

fn main() {
    let _ = part2();
}
