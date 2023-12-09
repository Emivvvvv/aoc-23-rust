use std::{fs, usize};
use std::cmp::min;
use std::collections::HashMap;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let lines_vector = input.lines().collect::<Vec<_>>();

    //part1
    let key_order_vec: Vec<&str> = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:"];

    let mut map: HashMap<&str, HashMap<String, (usize, usize, usize)>> = HashMap::new();

    let seeds: Vec<&str> = lines_vector[0].split(" ").filter(|&x| x != "" && x != "seeds:").collect();

    for (line_index, line) in lines_vector.iter().enumerate() {
        if line == &"" {
            let mut new_index = line_index + 1;
            let key = lines_vector[new_index];
            map.insert(key, HashMap::new());
            while  new_index + 1 < lines_vector.len() && lines_vector[new_index + 1] != ""{
                new_index += 1;
                let values = lines_vector[new_index].split(" ").collect::<Vec<&str>>();
                map.get_mut(key).unwrap().insert(String::from(values[1].to_owned() + "-" + values[2]), (values[1].parse::<usize>().unwrap(), values[0].parse::<usize>().unwrap(), values[2].parse::<usize>().unwrap()));
            }
        } else {
            continue
        }
    }

    let mut seed_and_min_loc = (0usize, usize::MAX);

    for seed in &seeds {
        let seed_usize = seed.parse::<usize>().unwrap();
        let mut new_key = seed_usize;
        for key in &key_order_vec {
            for (_, (source, destination, range)) in &map[key] {
                if new_key <= source + range && new_key >= *source {
                    new_key = destination + new_key - source;
                    break
                }
            }
        }
        if new_key < seed_and_min_loc.1 {
            seed_and_min_loc = (seed_usize, new_key)
        }
    }

    println!("Seed number and location: {} {} (part1)", seed_and_min_loc.0, seed_and_min_loc.1);

    //part2
    seed_and_min_loc = (0usize, usize::MAX);

    let mut clone_reverse_key_order=  key_order_vec.clone();
    clone_reverse_key_order.reverse();

    let mut min_loc = usize::MAX;
    let mut min_source = 0usize;
    let mut min_range = 0usize;
    for (_, (source, destination, range)) in &map[clone_reverse_key_order.remove(0)] {
        if *destination < min_loc {
            min_loc = *destination;
            min_source = *source;
            min_range = *range;
        }
    }

    let mut old_source = min_source;
    let mut old_range = min_range;

    for key in &clone_reverse_key_order {
        for (_, (source, destination, range)) in &map[key] {
            if *destination >= old_source && *destination <= old_source + old_range {
                old_source = *source;
                old_range = if destination + *range > old_source + old_range{
                    destination - (old_source + old_range)
                } else {
                    *range
                }
            }
        }
    }

    println!("{old_source} {old_range}");

    for seed in old_source..old_source + old_range {
        let mut new_key = seed;
        for key in &key_order_vec {
            for (_, (source, destination, range)) in &map[key] {
                if new_key <= source + range && new_key >= *source {
                    new_key = destination + new_key - source;
                    break
                }
            }
        }
        if new_key < seed_and_min_loc.1 {
            seed_and_min_loc = (seed, new_key)
        }
    }

    println!("Seed number and location: {} {} (part2)", seed_and_min_loc.0, seed_and_min_loc.1);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}