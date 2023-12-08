use std::fs;
use std::collections::HashMap;
use std::ops::Index;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let lines_vector = input.lines().collect::<Vec<_>>();

    //part1
    let mut total = 0;
    let mut flag;
    let line_count = lines_vector.len();
    let line_length = lines_vector[0].len();

    let mut row_index_number: Vec<(usize, usize, &str)> = Vec::new();
    for i in 0..lines_vector.len() {
        let edited_line = ".".to_owned() + lines_vector[i] + ".";
        let splitted_numbers: Vec<&str> = lines_vector[i]
            .split(&['.', '+', '=', '@', '#', '-', '*', '/' , '\\', '&', '$', '%'])
            .filter(|&x| x != "").collect();

        let mut found_numbers: Vec<&str> = Vec::new();

        for number in splitted_numbers.iter() {
            if !found_numbers.contains(number) {
                let all_occurrence: Vec<(usize, &str)> = edited_line.match_indices(number).collect();

                for occurrence in all_occurrence {
                    if !(edited_line.as_bytes()[occurrence.0 - 1] as char).is_ascii_digit() && !(edited_line.as_bytes()[occurrence.0 + occurrence.1.len()]as char).is_numeric() {
                        row_index_number.push((i, occurrence.0 - 1, number));
                    }
                }
                found_numbers.push(number);
            }
        }
    }


    for (row, index, number) in &row_index_number {
        let len = number.len();
        flag = false;

        for r in 0..=2 {
            if r == 0 && row == &0usize || r == 2 && row == &(line_count - 1) {
                continue
            }
            for c in 0..=len + 1 {
                if c == 0 && index == &0usize || c == len + 1 && index == &(line_length - len) {
                    continue
                }
                if (lines_vector[row + r - 1].as_bytes()[index + c - 1] as char).to_string().contains(&['+', '=', '@', '#', '-', '*', '/' , '\\', '&', '$', '%']) {
                    flag = true;
                }
            }
        }

        if flag {
            total += number.parse::<i32>().unwrap();
        }
    }

    println!("Total: {total} (part1)");

    //part2
    total = 0;
    let mut stars_row_index: HashMap<(usize, usize), Vec<usize>> = HashMap::new();


    for i in 0..lines_vector.len() {
        let all_occurrence: Vec<(usize, &str)> = lines_vector[i].match_indices('*').collect();
        for occurrence in all_occurrence {
            stars_row_index.insert((i, occurrence.0), vec![]);
        }
    }

    for (row, index, number) in &row_index_number {
        let len = number.len();
        let mut star_row = 0;
        let mut star_index = 0;
        flag = false;

        for r in 0..=2 {
            if r == 0 && row == &0usize || r == 2 && row == &(line_count - 1) {
                continue
            }
            for c in 0..=len + 1 {
                if c == 0 && index == &0usize || c == len + 1 && index == &(line_length - len) {
                    continue
                }
                if (lines_vector[row + r - 1].as_bytes()[index + c - 1] as char).to_string().contains(&['*']) {
                    star_row = row + r - 1;
                    star_index = index + c - 1;
                    stars_row_index.get_mut(&(star_row, star_index)).unwrap().push(number.parse::<usize>().unwrap());
                    println!("star found!")
                }
            }
        }
    }

    for values in stars_row_index.values() {
        let mut gear_value = 1usize;
        if values.len() <= 1 {
            continue
        } else {
            values.iter().for_each(|x| gear_value *= x );
        }

        total += gear_value as i32;
    }

    println!("Total: {total} (part2)");
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}