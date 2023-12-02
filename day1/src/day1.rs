use std::{fs, collections::HashMap};

fn decode_digits(str_in : &str) -> u32 {
    let digit_vec : Vec<u32> = str_in.chars()
                                     .filter(|c| c.is_digit(10))
                                     .map(|c| c.to_digit(10).unwrap())
                                     .collect();
    digit_vec.first().unwrap() * 10 + digit_vec.last().unwrap()
}

fn translate_words_to_numbers(str_in : &str) -> String {
    let mut str_in_replaced : String = str_in.to_string();

    // Letters can be reused... the spec helpfully did not mention this nor
    // provide an example that shows this...
    // Need to keep the letters at the start/end that can overlap with
    // adjacent spelled out words
    let numname_to_code_map : HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4ur"),
        ("five", "fi5e"),
        ("six", "si6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "ni9e"),
    ]);

    for (key, value) in numname_to_code_map {
        str_in_replaced = str_in_replaced.replace(key, value);
    }

    str_in_replaced
}

pub fn day_1_part_1() {
    let file_path = "input_1.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let sum : u32 = contents.lines()
                            .map(|line| decode_digits(line))
                            .sum();
    println!("Part 1 Result: {sum}");
}

pub fn day_1_part_2() {
    let file_path = "input_1.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let sum : u32 =  contents.lines().map(|line| translate_words_to_numbers(line))
                                     .map(|line| decode_digits(line.as_str()))
                                     .sum();
    println!("Part 2 Result: {sum}");                
}