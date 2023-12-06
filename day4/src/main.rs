use std::{collections::{HashSet, HashMap}, fs};
use regex::Regex;

struct Line {
    id : u8,
    winning_nums : HashSet<u8>,
    nums : HashSet<u8>
}

impl Line {
    fn get_nums_from_string(str_in : &str) -> HashSet<u8> {
        str_in.split(" ")
              .into_iter()
              .filter_map(|num_str| num_str.parse::<u8>().ok())
              .collect()
    }
    
    fn from_string(str_in : &str) -> Line {
        let line_re = Regex::new(r"Card *(\d+) *: *([\d ]+)\|([\d ]*)").unwrap();
        let match_re = line_re.captures_iter(str_in).next().unwrap();

        Line {
            id: match_re.get(1).unwrap().as_str().parse().unwrap(),
            winning_nums: Line::get_nums_from_string(match_re.get(2).unwrap().as_str()),
            nums: Line::get_nums_from_string(match_re.get(3).unwrap().as_str())
        }
    }

    fn get_num_matches(&self) -> u32 {
        self.winning_nums.intersection(&self.nums).count().try_into().unwrap()
    }

    fn get_score(&self) -> u32 {
        let num_matches : u32 = self.get_num_matches();

        if num_matches > 0 {
            u32::pow(2, num_matches-1)
        }
        else {
            0
        }
    }
}

fn part1() {
    let file_path = "input_4.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let points : u32 = contents.lines().map(|line| Line::from_string(line).get_score()).sum();
    println!("Part 1 Result: {points}");
}

fn part2() {
    let file_path = "input_4.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    // Construct a card id --> # matches map by parsing input data
    let card_id_to_matches_map : HashMap<u8, u32> = contents.lines()
                                                            .map(|line_str| { 
                                                                    let line = Line::from_string(line_str);
                                                                    (line.id, line.get_num_matches())
                                                                })
                                                            .collect();
    let num_ids = card_id_to_matches_map.len();

    // Initialize a map from id --> num_copies where every id starts off with 1 copy
    let mut card_id_to_copies_map : HashMap<u8, u32> = card_id_to_matches_map.iter().map(|entry| (*entry.0, 1)).collect();

    for card_id in  1..(num_ids+1) {
        let card_id_u8 = u8::try_from(card_id).unwrap();
        let num_matches = *card_id_to_matches_map.get(&card_id_u8).unwrap();
        let num_copies = *card_id_to_copies_map.get(&card_id_u8).unwrap();

        // Add the copies to the card_id_to_copies_map
        for copy_card_id in (card_id + 1)..(card_id + 1 + usize::try_from(num_matches).unwrap()) {
            *card_id_to_copies_map.get_mut(&u8::try_from(copy_card_id).unwrap()).unwrap() += num_copies;
        }
    }

    let num_cards : u32 = card_id_to_copies_map.iter().map(|entry| entry.1).sum();
    println!("Part 2 Result: {num_cards}");
}

fn main() {
    part1();
    part2();
}
