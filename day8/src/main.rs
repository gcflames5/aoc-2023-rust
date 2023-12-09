use regex::Regex;
use std::{collections::HashMap, fs, thread::current};

fn part1() {
    let file_path = "input_8.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut lines = contents.lines();
    let move_pattern: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next(); // blank line
    let node_map: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let line_re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
            let match_re = line_re.captures(line).unwrap();

            (
                match_re.get(1).unwrap().as_str(),
                (
                    match_re.get(2).unwrap().as_str(),
                    match_re.get(3).unwrap().as_str(),
                ),
            )
        })
        .collect();

    let mut current_location = "AAA";
    let mut move_iter = move_pattern.iter().cycle();
    let mut num_steps = 0;
    while current_location != "ZZZ" {
        let node_connections = node_map.get(current_location).unwrap();
        current_location = match move_iter.next().unwrap() {
            'L' => node_connections.0,
            'R' => node_connections.1,
            _ => panic!("big sad"),
        };
        num_steps += 1;
    }

    println!("Part 1 Result: {num_steps}");
}

fn part2() {
    let file_path = "input_8.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut lines = contents.lines();
    let move_pattern: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next(); // blank line

    let node_map: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let line_re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
            let match_re = line_re.captures(line).unwrap();

            (
                match_re.get(1).unwrap().as_str(),
                (
                    match_re.get(2).unwrap().as_str(),
                    match_re.get(3).unwrap().as_str(),
                ),
            )
        })
        .collect();

    let mut current_nodes : Vec<&str> = node_map
        .iter()
        .map(|entry| *entry.0)
        .filter(|node| node.ends_with('A'))
        .collect();
    dbg!(&current_nodes);

    let mut move_iter = move_pattern.iter().cycle();
    let mut num_steps = 0;
    while !current_nodes.iter().all(|node| { /*println!("{} {}", node, node.ends_with('Z'));*/ node.ends_with('Z') }) {
        if current_nodes.iter().filter(|node| node.contains('Z')).count() > 2 {
            dbg!(&current_nodes);
        }
        //dbg!(&current_nodes);
        let chosen_move = move_iter.next().unwrap(); 
        current_nodes = current_nodes.iter().map(|node| {
            let node_connections = node_map.get(node).unwrap();
            match chosen_move {
                'L' => node_connections.0,
                'R' => node_connections.1,
                _ => panic!("big sad"),
            }
        }).collect();

        num_steps += 1;

        if (num_steps % 1000000 == 0) {
            //break;
            dbg!(&num_steps);
            println!("{chosen_move}");
        }
    }

    println!("Part 2 Result: {num_steps}");
}

fn main() {
    //part1();
    part2();
}
