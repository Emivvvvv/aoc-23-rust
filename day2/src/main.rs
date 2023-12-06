use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let lines_vector = input.lines().collect::<Vec<_>>();

    let mut total = 0;

    //part1
    for game in &lines_vector {
        let hands: Vec<&str> = game
            .split(&[':',';']).collect();
        let mut hands_detailed = hands
            .iter()
            .map(|&x| x.split(&[' ', ',']).filter(|&x| x != "").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let game_number = hands_detailed.remove(0)[1];

        let mut flag = true;
        for hand in &hands_detailed {
            for i in 0..hand.len() - 1 {
                if hand[i + 1] == "red" && hand[i].parse::<i32>().unwrap() > 12
                    || hand[i + 1] == "green" && hand[i].parse::<i32>().unwrap() > 13
                    || hand[i + 1] == "blue" && hand[i].parse::<i32>().unwrap() > 14{
                    flag = false;
                }
            }
        }

        if flag {
            total += game_number.parse::<i32>().unwrap()
        }
    }

    println!("Total {total} (part1)");

    //part2
    total = 0;
    for game in &lines_vector {
        let hands: Vec<&str> = game
            .split(&[':',';']).collect();
        let mut hands_detailed = hands
            .iter()
            .map(|&x| x.split(&[' ', ',']).filter(|&x| x != "").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        hands_detailed.remove(0);

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for hand in &hands_detailed {
            for i in 0..hand.len() - 1 {
                if hand[i + 1] == "red" && hand[i].parse::<i32>().unwrap() > red {
                    red = hand[i].parse::<i32>().unwrap()
                } else if hand[i + 1] == "green" && hand[i].parse::<i32>().unwrap() > green {
                    green = hand[i].parse::<i32>().unwrap()
                } else if hand[i + 1] == "blue" && hand[i].parse::<i32>().unwrap() > blue {
                    blue = hand[i].parse::<i32>().unwrap()
                }
            }
        }

        total += red * green * blue
    }

    println!("Total {total} (part2)");
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}