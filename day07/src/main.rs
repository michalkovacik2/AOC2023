use std::cmp::Ordering;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use std::iter::zip;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Pre Enum sa daju definovat Traits pre porovnanie, celkom COOL
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighHand,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<i8>,
    bid: i32,
    hand_type: EHandType,
}

impl Hand {
    fn determine_hand_type(cards: &Vec<i8>) -> EHandType {
        let mut copy_cards = cards.clone();
        let mut card_counts: Vec<(i8, usize)> = Vec::new();

        while !copy_cards.is_empty() {
            let current_card = copy_cards[0];
            let current_card_count = copy_cards.iter().filter(|&card| { *card == current_card }).count();
            card_counts.push((current_card, current_card_count));
            copy_cards.retain(|card| { *card != current_card });
        }

        card_counts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Erase JOKERS and use them as wild cards for best cards
        let mut number_of_jokers = 0;
        for card_count in card_counts.iter() {
            if card_count.0 == 1 {
                number_of_jokers = card_count.1;
            }
        }
        card_counts.retain(|x| { x.0 != 1 });
        if card_counts.is_empty() {
            // Handle special case when we have JJJJJ (5 JOKERS)
            card_counts.push((1, 5));
        } else {
            card_counts[0].1 += number_of_jokers;
        }

        if card_counts[0].1 == 5 {
            return EHandType::FiveOfAKind;
        } else if card_counts[0].1 == 4 {
            return EHandType::FourOfAKind;
        } else if card_counts[0].1 == 3 && card_counts[1].1 == 2 {
            return EHandType::FullHouse;
        } else if card_counts[0].1 == 3 && card_counts[1].1 == 1 {
            return EHandType::ThreeOfAKind;
        } else if card_counts[0].1 == 2 && card_counts[1].1 == 2 {
            return EHandType::TwoPair;
        } else if card_counts[0].1 == 2 && card_counts[1].1 == 1 {
            return EHandType::OnePair;
        } else {
            return EHandType::HighHand;
        }
    }
    
    pub fn new(cards: Vec<i8>, bid: i32) -> Self {
        let hand_type = Self::determine_hand_type(&cards);
        Self {
            cards: cards,
            bid: bid,
            hand_type: hand_type,
        }
    }
}

// Custom implementations for ordering 
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Greater;
        } else if self.hand_type > other.hand_type {
            return Ordering::Less;
        } else {
            for (a, b) in zip(&self.cards, &other.cards) {
                if a < b {
                    return Ordering::Less;
                } else if b < a {
                    return Ordering::Greater;
                } else {
                    continue;
                }
            }
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {
}

fn parse_input(file_name: &str, use_joker: bool) -> Vec<Hand> {
    let card_values_map: HashMap<char, i8> = HashMap::from([
        ('*',  1), // JOKER
        ('2',  2),
        ('3',  3),
        ('4',  4),
        ('5',  5),
        ('6',  6),
        ('7',  7),
        ('8',  8),
        ('9',  9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let mut hands: Vec<Hand> = Vec::new();

    if let Ok(file_lines) = read_lines(file_name) {
        for file_line in file_lines {
            if let Ok(line) = file_line {
                let splitted: Vec<&str> = line.split(" ").collect();
                let mut cards_str = splitted[0].to_owned();
                if use_joker {
                    cards_str = cards_str.replace("J", "*");
                }
                let mut cards: Vec<i8> = Vec::new();
                cards_str.chars().for_each(|card|{
                    cards.push(*card_values_map.get(&card).unwrap());
                });
                let bid = i32::from_str_radix(splitted[1].trim(), 10).unwrap();
                hands.push(Hand::new(cards, bid));
            }
        }
    }

    hands
}

fn part1_and_part2(file_name: &str, use_joker: bool) -> io::Result<()> {
    let mut total_winnings: i64 = 0;
    let mut hands: Vec<Hand> = parse_input(file_name, use_joker);
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (i as i64 + 1) * hand.bid as i64;
    }

    println!("Total winnings: {}", total_winnings);
    Ok(())
}

fn main() {
    // let _ = part1_and_part2("example.txt", false);
    let _ = part1_and_part2("input.txt", true);
}
