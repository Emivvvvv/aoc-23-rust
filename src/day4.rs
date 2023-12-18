use std::collections::HashMap;

pub fn answers(input: String) -> Vec<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let results: Vec<String> = vec![part1(&lines), part2(&lines)];

    return results;
}

fn part1(lines: &Vec<&str>) -> String {
    let mut total = 0;
    for card in lines {
        let splitted: Vec<&str> = card.split(" ").filter(|&x| x != "" && x != "Card" && !x.contains(':')).collect();
        let seperator_index = splitted.iter().position(|&x|x == "|").unwrap();
        let winning_nums: &[&str] = &splitted[..seperator_index].to_vec();
        let card_nums: &[&str] = &splitted[seperator_index + 1..].to_vec();

        let mut winning_num_count = 0;
        winning_nums.iter().for_each(|&winning_num| winning_num_count += card_nums.iter().filter(|&&x| x == winning_num).count());

        if winning_num_count == 0 {
            continue
        } else {
            total += 2_i32.pow(winning_num_count as u32 - 1)
        }
    }

    return total.to_string()
}

fn part2(lines: &Vec<&str>) -> String {
    let mut total = 0;

    let mut scratchcards: HashMap<usize, (usize, usize),> = HashMap::new();

    for card in lines {
        let splitted: Vec<&str> = card.split(" ").filter(|&x| x != "" && x != "Card").collect();
        let seperator_index = splitted.iter().position(|&x| x == "|").unwrap();
        let winning_nums: &[&str] = &splitted[1..seperator_index].to_vec();
        let card_nums: &[&str] = &splitted[seperator_index + 1..].to_vec();

        let mut winning_num_count = 0;
        winning_nums.iter().for_each(|&winning_num| winning_num_count += card_nums.iter().filter(|&&x| x == winning_num).count());

        let i = splitted[0].replace(":", "").parse::<usize>().unwrap() - 1;
        scratchcards.insert(i, (winning_num_count, 1));
    }

    for i in 0..scratchcards.len() {
        let (winning_num_count, card_count) = scratchcards[&i];
        for j in 0..winning_num_count {
            let (winning_num_count_save, card_count_save) = scratchcards[&(i + j + 1)];
            scratchcards.insert(i + j + 1, (winning_num_count_save, card_count_save + card_count));
        }

        total += card_count as i32;
    }

    return total.to_string()
}