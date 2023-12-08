use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    Invalid = -1,
    High = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn get_card_power(card: char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("guh^2"),
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bet: u32,
    hand_type: HandType,
}

impl Hand {
    fn from_string(str_in: &str) -> Hand {
        let mut whitespace_iter = str_in.split_whitespace();
        let cards = whitespace_iter.next().unwrap().chars().collect();
        let bet = whitespace_iter.next().unwrap().parse().unwrap();

        Hand {
            bet: bet,
            hand_type: Hand::get_hand_type(&cards),
            cards: cards,
        }
    }

    fn get_hand_type(cards: &Vec<char>) -> HandType {
        assert!(cards.len() == 5);

        // Construct map of cards --> frequency
        let mut freq_map: HashMap<char, i8> = HashMap::new();
        for card in cards.iter() {
            *freq_map.entry(*card).or_insert(0) += 1;
        }

        let highest_freq = freq_map.iter().map(|entry| entry.1).max().unwrap();
        let freq_map_len = freq_map.len();

        match highest_freq {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => {
                assert!(freq_map_len == 2 || freq_map_len == 3);
                if freq_map_len == 2 {
                    // Only 2 types of card, other cards must match
                    return HandType::FullHouse;
                } else {
                    return HandType::ThreeOfAKind;
                }
            }
            2 => {
                assert!(freq_map_len == 3 || freq_map_len == 4);
                if freq_map_len == 3 {
                    // Only 3 types of card, must be another pair
                    return HandType::TwoPair;
                } else {
                    return HandType::OnePair;
                }
            }
            1 => return HandType::High,
            _ => panic!("guh"),
        }
    }

    fn get_card_power(&self) -> u32 {
        self.cards
            .iter()
            .rev()
            .map(|card| get_card_power(*card))
            .enumerate()
            .map(|(pos, card_power)| {
                u32::pow(15, u32::try_from(pos).unwrap()) * u32::try_from(card_power).unwrap()
            })
            .sum()
    }
}

fn part1() {
    let file_path = "input_7.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|line| Hand::from_string(line))
        .collect();
    hands.sort_by_key(|hand| (hand.hand_type, hand.get_card_power()));
    let bet_sum : u32 = hands
        .iter()
        .enumerate()
        .map(|(pos, hand)| (u32::try_from(pos).unwrap() + 1) * hand.bet)
        .sum();

    println!("Part 1 Result: {bet_sum}");
}

fn main() {
    part1();
}
