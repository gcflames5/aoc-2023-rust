use std::fs;

fn line_to_vec(str_in: &str) -> Vec<u64> {
    str_in
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn calc_score(times: &Vec<u64>, records: &Vec<u64>) -> usize {
    times
        .iter()
        .zip(records)
        .map(|(time, record)| (1..*time).filter(|n| (time - n) * n > *record).count())
        .fold(1, |acc, elem| acc * elem)
}

fn part1() {
    let file_path = "input_6.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut contents_lines = contents.lines();

    let times = line_to_vec(contents_lines.next().unwrap());
    let records = line_to_vec(contents_lines.next().unwrap());

    println!("Part 1 Result: {}", calc_score(&times, &records));
}

fn part2() {
    let file_path = "input_6.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut contents_lines = contents.lines();

    let times = line_to_vec(contents_lines.next().unwrap().replace(' ', "").as_str());
    let records = line_to_vec(contents_lines.next().unwrap().replace(' ', "").as_str());

    println!("Part 2 Result: {}", calc_score(&times, &records));
}

fn main() {
    part1();
    part2();
}
