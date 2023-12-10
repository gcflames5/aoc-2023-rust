use std::fs;

fn get_next_value(vec_in: &Vec<i32>) -> i32 {
    let mut diff_lists: Vec<Vec<i32>> = Vec::new();
    diff_lists.push(vec_in.clone());

    // Keep making difference lists until its all zeros
    while !diff_lists.last().unwrap().iter().all(|entry| *entry == 0) {
        diff_lists.push(
            diff_lists
                .last()
                .unwrap()
                .windows(2)
                .map(|win| win[1] - win[0])
                .collect(),
        );
    }

    // Now add a 0 to the bottom diff list and propogate upwards  
    (0..diff_lists.len()).rev().for_each(|list_id| {
        let diff_below : i32 = match diff_lists.get(list_id + 1) {
            Some(below_vec) => *below_vec.last().unwrap(),
            _ => 0
        };
        let mut my_list : &mut Vec<i32> = diff_lists.get_mut(list_id).unwrap();
        (*my_list).push(my_list.last().unwrap() + diff_below);
    });

    *diff_lists.get(0).unwrap().last().unwrap()
}

fn part1() {
    let file_path = "input_9.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let line_vecs: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();
    let next_values : Vec<i32> = line_vecs.iter().map(|vec| get_next_value(vec)).collect();
    let p1_result : i32 = next_values.iter().sum();

    println!("Part 1 Result: {p1_result}");
}

fn main() {
    part1();
}
