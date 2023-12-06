use std::{collections::HashMap, fs, str::Lines};

use regex::Regex;

#[derive(Debug)]
struct MapEntry {
    from_start: u64,
    to_start: u64,
    size: u64,
}

impl MapEntry {
    fn from_string(str_in: &str) -> MapEntry {
        let mut num_iter = str_in.split_whitespace();

        MapEntry {
            to_start: num_iter.next().unwrap().parse().unwrap(),
            from_start: num_iter.next().unwrap().parse().unwrap(),
            size: num_iter.next().unwrap().parse().unwrap(),
        }
    }

    fn reverse(&self) -> MapEntry {
        MapEntry {
            from_start: self.to_start,
            to_start: self.from_start,
            size: self.size,
        }
    }
}
#[derive(Debug)]
struct Mapping {
    map_to: String,
    mappings: Vec<MapEntry>,
}

impl Mapping {
    fn from_str_iter(map_to_name: &str, iter: &mut Lines) -> Mapping {
        let mut mappings = Vec::new();

        for line in iter {
            if line.len() < 2 {
                break;
            }

            mappings.push(MapEntry::from_string(line));
        }

        Mapping {
            map_to: map_to_name.to_string(),
            mappings: mappings,
        }
    }
}

fn get_corresponding_entry(input_num: u64, mapping: &Mapping) -> u64 {
    let mut mappings_iter = mapping.mappings.iter();
    while let Some(map_entry) = mappings_iter.next() {
        if input_num >= map_entry.from_start && input_num <= map_entry.from_start + map_entry.size {
            return (input_num - map_entry.from_start) + map_entry.to_start;
        }
    }
    return input_num;
}

fn find_attribute_for(
    starting_attribute: &str,
    starting_attribute_value: u64,
    target_attribute: &str,
    maps: &HashMap<&str, Mapping>,
) -> u64 {
    let mut current_attribute = starting_attribute;
    let mut current_value: u64 = starting_attribute_value;

    // This could probably use .fold() instead
    while !current_attribute.eq(target_attribute) {
        let map_entry = maps.get(current_attribute).unwrap();

        current_value = get_corresponding_entry(current_value, map_entry);
        current_attribute = map_entry.map_to.as_str();
    }

    current_value
}

fn main() {
    let file_path = "input_5.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut line_iter = contents.lines();

    // Parse the seeds
    let seeds: Vec<u64> = line_iter
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|num_str| num_str.parse::<u64>().ok())
        .collect();

    // Construct a Hashmap mapping strings --> MapEntrys
    // For example a string "water" would be mapped to a { "fertilzer", [Mapping] }
    let mut maps: HashMap<&str, Mapping> = HashMap::new();

    while let Some(line) = line_iter.next() {
        if line.contains("map") {
            let map_re = Regex::new(r"([a-zA-Z]+)-to-([a-zA-Z]+).*").unwrap();
            let match_re = map_re.captures(line).unwrap();
            let (map_from, map_to) = (
                match_re.get(1).unwrap().as_str(),
                match_re.get(2).unwrap().as_str(),
            );

            let mapping = Mapping::from_str_iter(map_to, &mut line_iter);
            maps.insert(map_from, mapping);
        }
    }

    // Now use the map to loop through each seed and get the requested attribute
    let min_loc = seeds
        .iter()
        .map(|seed| find_attribute_for("seed", *seed, "location", &maps))
        .min()
        .unwrap();

    println!("Part 1 Result: {min_loc}");

    // Mirror the p1 mappings, but swap the from and to strings
    let p2_maps: HashMap<&str, Mapping> = maps
        .iter()
        .map(|entry| {
            (
                entry.1.map_to.as_str(),
                Mapping {
                    map_to: entry.0.to_string(),
                    mappings: entry
                        .1
                        .mappings
                        .iter()
                        .map(|map_entry| map_entry.reverse())
                        .collect(),
                },
            )
        })
        .collect();

    // Search bacwards, starting from locations and working towards seeds
    // After we convert the location number to a seed number, check if that seed # is in the list of seeds
    let mut loc_num: u64 = 0;
    loop {
        // Find the seed # that corresponds to this location
        let seed = find_attribute_for("location", loc_num, "seed", &p2_maps);
        
        // Break seed list into chunks of 2, where the first is the base and the second is length
        // Then check if any match is found for the seed
        let match_found= seeds
            .chunks(2)
            .any(|chunk| { seed >= chunk[0] && seed <= (chunk[0] + chunk[1]) });

        if match_found {
            println!("Part 2 Result: {loc_num}");
            break;
        }

        loc_num += 1;
    }
}
