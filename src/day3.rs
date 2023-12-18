use std::collections::HashMap;

pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = part1_and_2(&lines);

    return results;
}

fn part1_and_2(lines: &Vec<&str>) -> Vec<String> {
    let mut total = 0;
    let mut flag;
    let line_count = lines.len();
    let line_length = lines[0].len();

    let mut row_index_number: Vec<(usize, usize, &str)> = Vec::new();
    for i in 0..lines.len() {
        let edited_line = ".".to_owned() + lines[i] + ".";
        let splitted_numbers: Vec<&str> = lines[i]
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
                if (lines[row + r - 1].as_bytes()[index + c - 1] as char).to_string().contains(&['+', '=', '@', '#', '-', '*', '/' , '\\', '&', '$', '%']) {
                    flag = true;
                }
            }
        }

        if flag {
            total += number.parse::<i32>().unwrap();
        }
    }


    //part2
    let mut total2 = 0;
    let mut stars_row_index: HashMap<(usize, usize), Vec<usize>> = HashMap::new();


    for i in 0..lines.len() {
        let all_occurrence: Vec<(usize, &str)> = lines[i].match_indices('*').collect();
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
                if (lines[row + r - 1].as_bytes()[index + c - 1] as char).to_string().contains(&['*']) {
                    star_row = row + r - 1;
                    star_index = index + c - 1;
                    stars_row_index.get_mut(&(star_row, star_index)).unwrap().push(number.parse::<usize>().unwrap());
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

        total2 += gear_value as i32;
    }


    return vec![total.to_string(), total2.to_string()]
}