pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = vec![part1(&lines), part2(&lines)];

    return results;
}

fn part1(lines: &Vec<&str>) -> String {
    let times = lines[0]
        .split(" ")
        .filter(|&x| x != "" && x!= "Time:")
        .collect::<Vec<&str>>();
    let distances = lines[1]
        .split(" ")
        .filter(|&x| x != "" && x!= "Distance:").collect::<Vec<&str>>();

    let mut multiplied_num_ways = 1;

    for i in 0..times.len() {
        let mut num_ways = 0;
        let time = times[i].parse::<usize>().unwrap();
        let distance = distances[i].parse::<usize>().unwrap();
        for secs in 0..time {
            if secs * (time - secs) > distance {
                num_ways += 1
            }
        }
        multiplied_num_ways *= num_ways
    }

    return multiplied_num_ways.to_string()
}

fn part2(lines: &Vec<&str>) -> String {
    let times = lines[0]
        .split(" ")
        .filter(|&x| x != "" && x!= "Time:")
        .collect::<Vec<&str>>();
    let distances = lines[1]
        .split(" ")
        .filter(|&x| x != "" && x!= "Distance:").collect::<Vec<&str>>();

    let mut num_ways = 0;

    let time_2 = times.join("").parse::<usize>().unwrap();
    let distance_2 = distances.join("").parse::<usize>().unwrap();
    for secs in 0..time_2 {
        if secs * (time_2 - secs) > distance_2 {
            num_ways += 1
        }
    }

    return num_ways.to_string()
}