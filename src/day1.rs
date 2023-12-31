pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = vec![part1(&lines), part2(&lines)];

    return results;
}

fn part1(lines: &Vec<&str>) -> String {
    let mut total: u32 = 0;

    for line in lines {
        let line_nums: Vec<u32> = line
            .chars()
            .filter(|x| x.is_numeric())
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        total += line_nums.first().unwrap_or(&0) * 10 + line_nums.last().unwrap_or(&0);
    }

    return total.to_string()
}

fn part2(lines: &Vec<&str>) -> String {
    let mut total = 0;
    for line in lines {

        let line_vec: Vec<char> = line.chars().collect();

        let mut edited_only_nums: Vec<char> = Vec::new();

        for (i, char) in line_vec.iter().enumerate() {
            if char.is_numeric() {
                edited_only_nums.push(*char);
            }
            if i + 2 < line_vec.len() {
                if char == &'o' && line_vec[i + 1..= i + 2] == ['n', 'e'] {
                    edited_only_nums.push('1')
                } else if char == &'t' && line_vec[i + 1..= i + 2] == ['w', 'o'] {
                    edited_only_nums.push('2')
                } else if char == &'s' && line_vec[i + 1..= i + 2] == ['i', 'x'] {
                    edited_only_nums.push('6')
                }
            }
            if i + 3 < line_vec.len() {
                if char == &'f' && line_vec[i + 1..= i + 3] == ['o', 'u', 'r'] {
                    edited_only_nums.push('4')
                } else if char == &'f' && line_vec[i + 1..= i + 3] == ['i', 'v', 'e'] {
                    edited_only_nums.push('5')
                } else if char == &'n' && line_vec[i + 1..= i + 3] == ['i', 'n', 'e'] {
                    edited_only_nums.push('9')
                }
            }
            if i + 4 < line_vec.len() {
                if char == &'t' && line_vec[i + 1..=i + 4] == ['h', 'r', 'e', 'e'] {
                    edited_only_nums.push('3')
                } else if char == &'s' && line_vec[i + 1..=i + 4] == ['e', 'v', 'e', 'n'] {
                    edited_only_nums.push('7')
                } else if char == &'e' && line_vec[i + 1..=i + 4] == ['i', 'g', 'h', 't'] {
                    edited_only_nums.push('8')
                }
            }
        }

        total +=
            edited_only_nums.first().unwrap().to_digit(10).unwrap() * 10 +
                edited_only_nums.last().unwrap().to_digit(10).unwrap()
    }

    return total.to_string()
}