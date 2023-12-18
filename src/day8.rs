use std::collections::HashMap;

pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = part1_and_2(&lines);

    return results;
}

fn part1_and_2(lines: &Vec<&str>) -> Vec<String> {
    let instructions = lines[0].chars().collect::<Vec<char>>();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines[2..].iter() {
        let splitted_line = line.split(&['=', ',', '(', ')', ' ']).filter(|&x| x != "").collect::<Vec<&str>>();
        map.insert(splitted_line[0], (splitted_line[1], splitted_line[2]));
    }

    let mut current_loc = "AAA";
    let mut total_steps = 0;
    loop {
        for instruction in &instructions {
            if instruction == &'L' {
                current_loc = map[current_loc].0
            } else if instruction == &'R' {
                current_loc = map[current_loc].1
            }
        }
        total_steps += instructions.len();
        if current_loc == "ZZZ" {
            break
        }
    }

    let part1_answer = total_steps;

    //part2
    let mut starting_locs: Vec<&str> = Vec::new();
    let mut least_steps: Vec<usize> = Vec::new();
    total_steps = 0;

    for key in map.keys() {
        if key.chars().nth(2).unwrap() == 'A' {
            starting_locs.push(key)
        }
    }

    for starting_loc in starting_locs {
        let mut current_loc = starting_loc;
        total_steps = 0;
        loop {
            for instruction in &instructions {
                if instruction == &'L' {
                    current_loc = map[current_loc].0
                } else if instruction == &'R' {
                    current_loc = map[current_loc].1
                }
            }
            total_steps += instructions.len();
            if current_loc.chars().nth(2).unwrap() == 'Z' {
                least_steps.push(total_steps);
                break
            }
        }
    }

    println!("DAY8 NOTE: Get the lowest common multiple of those numbers from a random website: {:?}", least_steps);

    return vec![part1_answer.to_string(), "Calculate LCM".to_string()]
}
