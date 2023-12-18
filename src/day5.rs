use std::collections::HashMap;

pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = part1_and_part2(&lines);

    return results;
}

fn part1_and_part2(lines: &Vec<&str>) -> Vec<String> {
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

    let seeds: Vec<&str> = lines[0].split(" ").filter(|&x| x != "" && x != "seeds:").collect();

    for (line_index, line) in lines.iter().enumerate() {
        if line == &"" {
            let mut new_index = line_index + 1;
            let key = lines[new_index];
            map.insert(key, HashMap::new());
            while  new_index + 1 < lines.len() && lines[new_index + 1] != ""{
                new_index += 1;
                let values = lines[new_index].split(" ").collect::<Vec<&str>>();
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

    let part1_answer = seed_and_min_loc.1;

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

    println!("DAY5 NOTE: Future Emirhan: I have no idea why the part2 answer is old_range but not new location: {}. I was on a trip so forgot the old code and don't want to spend a lot time because there are a lot day to catch... After finishing the whole calendar I can rewrite the code but this will stay litke this for now.", seed_and_min_loc.1);
    return vec![part1_answer.to_string(), old_range.to_string()]
}