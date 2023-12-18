use std::collections::HashMap;

pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = part1(&lines);

    return results;
}

fn part1(lines: &Vec<&str>) -> Vec<String> {
    let mut total = 0;
    let mut reversed_total = 0;

    //part 1 and part2
    //part 2 solution is basically same but the input is reversed.
    for line in lines {
        let mut splitted_line = line
            .split(" ")
            .filter(|&x| x != "")
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut reversed_line = splitted_line.clone();
        reversed_line.reverse();

        let splitted_lines = vec![splitted_line, reversed_line];

        let mut part = 0;

        for splitted_line in splitted_lines
        {
            part += 1;
            let mut map: HashMap<usize, Vec<i32>> = HashMap::new();


            let mut i = 0;
            map.insert(i, splitted_line);

            loop {
                let mut new_line_vector = Vec::new();
                let len = map[&i].len();
                for j in 0..len - 1 {
                    new_line_vector.push(map[&i][j + 1] - map[&i][j])
                }
                i += 1;
                map.insert(i, new_line_vector);

                if map[&i].iter().filter(|&x| x != &0_i32 ).count() == 0 {
                    break
                }
            }

            let map_len = map.len();

            map.get_mut(&(map_len - 1)).unwrap().push(0);
            let mut len_of_line = map[&(map_len - 1)].len();

            let mut c = 0;
            for i in 0..map_len - 1 {
                let a = map[&(map_len - i - 1)][len_of_line - 1];
                let b = map[&(map_len - i - 2)][len_of_line - 1];
                c = a + b;
                map.get_mut(&(map_len - i - 2)).unwrap().push(c);
                len_of_line = map[&(map_len - i - 2)].len()
            }

            if part == 1 {
                total += c;
            } else if part == 2 {
                reversed_total += c;
            }
        }
    }

    return vec![total.to_string(), reversed_total.to_string()]
}