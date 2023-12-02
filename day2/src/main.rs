use std::fs;

#[derive(Debug)]
struct Round {
    red : u32,
    green : u32, 
    blue : u32,
}

impl Default for Round {
    fn default () -> Round {
        Round{red: 0, green: 0, blue:0}
    }
}

impl Round {
    fn subset_of(&self, other_round : &Round) -> bool {
        self.red <= other_round.red
        && self.green <= other_round.green
        && self.blue <= other_round.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id : u32,
    rounds : Vec<Round>,
}

impl Game {
    fn get_min_round(&self) -> Round {
        let mut min_round : Round = {Default::default()};

        for round in self.rounds.iter() {
            if round.red >= min_round.red {
                min_round.red = round.red;
            }
            if round.green >= min_round.green {
                min_round.green = round.green;
            }
            if round.blue >= min_round.blue {
                min_round.blue = round.blue;
            }
        }

        min_round
    }
}

fn extract_digits(input : &str) -> u32 {
    input.chars()
         .filter(|c| c.is_digit(10))
         .collect::<String>()
         .parse().unwrap()
}

fn parse_round(round_str : &str) -> Round {
    let color_str_iter = round_str.split(",").into_iter();
    let mut round : Round = {Default::default()};

    for color_str in color_str_iter {
        let num = extract_digits(color_str);
        if color_str.contains("red") {
            round.red = num;
        }
        else if color_str.contains("blue") {
            round.blue = num;
        }
        else if color_str.contains("green") {
            round.green = num;
        }
    }

    round
} 

fn parse_game(line: &str) -> Game {
    let mut colon_split_iter = line.split(":").into_iter();
    
    Game { id:     extract_digits(colon_split_iter.next().unwrap()), 
           rounds: colon_split_iter.next().unwrap().split(";").map(|round_str| parse_round(round_str)).collect()
    }
} 

fn part1() {
    let file_path = "input_2.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let max_round = Round { red: 12, green: 13, blue: 14 };

    let sum : u32 =  contents.lines().map(|line| parse_game(line))
                                     .filter(|game| game.rounds.iter().all(|round| round.subset_of(&max_round)))
                                     .map(|game| game.id)
                                     .sum();
    println!("Part 1 Result: {sum}");           
}

fn part2() {
    let file_path = "input_2.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let sum : u32 =  contents.lines().map(|line| parse_game(line).get_min_round().power())
                                     .sum();
    println!("Part 2 Result: {sum}");       
}

fn main() {
      part1();
      part2();
}
