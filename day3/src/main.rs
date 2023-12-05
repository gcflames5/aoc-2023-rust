use std::{collections::{HashSet, VecDeque}, fs};
use regex::Regex;

fn get_symbol_set(line: &str) -> HashSet<usize> {
    let symbol_re = Regex::new(r"[^\d.\n\r]").unwrap();
    let mut output_set : HashSet<usize> = HashSet::new();
    
    for sym_match in symbol_re.captures_iter(line) {
        output_set.insert(sym_match.get(0).unwrap().start());
    }
             
    output_set
}

fn get_line_value(line_idx : usize, line : &str, symbols : &Vec<HashSet<usize>>) -> usize {
    // Get number matches
    let num_re = Regex::new(r"\d+").unwrap();
    let mut accum : usize = 0;

    for num_match in num_re.captures_iter(line) {
        let match_0 = num_match.get(0).unwrap();
        let (start_idx, end_idx) = (match_0.start(), match_0.end());
        let mut has_symbol = false;

        for num_idx in start_idx..end_idx {
            // Check if any symbols around
            // Above
            let above_idx = line_idx + 1;
            if above_idx < symbols.len() {
                for sym_idx_offset in 0..3 {
                    if sym_idx_offset != 0 || num_idx > 0 {
                        if symbols.get(above_idx).unwrap().contains(&(num_idx + sym_idx_offset - 1)) {
                            has_symbol = true;
                            break;
                        }
                    }
                }
            }

            // Below
            if line_idx > 0 {
                let below_idx = line_idx - 1;
                if below_idx < symbols.len() {
                    for sym_idx_offset in 0..3 {
                        if sym_idx_offset != 0 || num_idx > 0 {
                            if symbols.get(below_idx).unwrap().contains(&(num_idx + sym_idx_offset - 1)) {
                                has_symbol = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Adjacent
        if ((start_idx > 0) && symbols.get(line_idx).unwrap().contains(&(start_idx-1)))
            || symbols.get(line_idx).unwrap().contains(&(end_idx)) {
                has_symbol = true;
        }
        
        if has_symbol {
            accum += match_0.as_str().parse::<usize>().unwrap();
        }
    }

    accum
}

fn part1() {
    let file_path = "input_3.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let symbol_set_vec : Vec<HashSet<usize>> = contents.lines().map(|line| get_symbol_set(line)).collect();
    let total_value : usize = contents
                                .lines()
                                .enumerate()
                                .map(|(line_idx, line)| get_line_value(line_idx, line, &symbol_set_vec))
                                .sum();

    println!("Part1 Result: {total_value}");
}

fn main() {
    part1();
}
