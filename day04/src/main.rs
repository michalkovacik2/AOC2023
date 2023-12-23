use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Card {
    id : i32,
    winning_numbers : HashSet<i32>,
    my_numbers : Vec<i32>,
    count_matching_numbers: usize,
    count: usize,
}

fn parse_card(line: &str) -> Card {
    let mut card = Card {
        id : -1,
        winning_numbers: HashSet::new(), 
        my_numbers: Vec::new(),
        count_matching_numbers: 0,
        count: 1,
    };

    let splitted1: Vec<&str> = line.split(": ").collect();
    let name_splitted: Vec<&str> = splitted1[0].split("Card ").collect();
    card.id = i32::from_str_radix(name_splitted[1], 10).unwrap_or(-1);
    let numbers_splitted: Vec<&str> = splitted1[1].split(" | ").collect();
    for winning_number in numbers_splitted[0].split(" ") {
        if winning_number.len() > 0 {
            card.winning_numbers.insert(i32::from_str_radix(winning_number, 10).unwrap());
        }
    }
    for my_number in numbers_splitted[1].split(" ") {
        if my_number.len() > 0 {
            card.my_numbers.push(i32::from_str_radix(my_number, 10).unwrap());
        }
    }

    card
} 

fn part1_and_part2(file_name: &str) -> io::Result<()> {
    let mut total_points = 0;
    let mut total_cards = 0;
    let mut cards: Vec<Card> = Vec::new();
    // Karty su indexovane od 1, takze vlozim zlu kartu na zaciatok
    cards.push(Card {
        id : -1,
        winning_numbers: HashSet::new(), 
        my_numbers: Vec::new(),
        count_matching_numbers: 0,
        count: 0,
    });
    
    // PART 1
    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let mut card = parse_card(&line);
                // println!("{:#?}", card);
                let mut points = 0;
                for my_number in card.my_numbers.iter() {
                    if card.winning_numbers.contains(&my_number) {
                        points = if points == 0 { 1 } else { points * 2 };
                        card.count_matching_numbers += 1;
                    }
                }
                total_points += points;
                cards.push(card);
            }
        }
    }

    // PART 2
    for i in 1..cards.len() {
        for j in 0..cards[i].count_matching_numbers {
            cards[i + 1 + j].count += cards[i].count;
        }
    }

    cards.iter().for_each(|card|{
        total_cards += card.count;
    });

    println!("Total points: {}", total_points);
    println!("Total cards:  {}", total_cards);

    Ok(())
}

fn main() {
    let _ = part1_and_part2("input.txt");
}