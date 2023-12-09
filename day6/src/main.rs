use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let lines_vector = input.lines().collect::<Vec<_>>();

    let times = lines_vector[0]
        .split(" ")
        .filter(|&x| x != "" && x!= "Time:")
        .collect::<Vec<&str>>();
    let distances = lines_vector[1]
        .split(" ")
        .filter(|&x| x != "" && x!= "Distance:").collect::<Vec<&str>>();


    //part1

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

    println!("part1: {multiplied_num_ways}");

    //part2
    let mut num_ways = 0;

    let time_2 = times.join("").parse::<usize>().unwrap();
    let distance_2 = distances.join("").parse::<usize>().unwrap();
    for secs in 0..time_2 {
        if secs * (time_2 - secs) > distance_2 {
            num_ways += 1
        }
    }

    println!("part2: {num_ways}");
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}