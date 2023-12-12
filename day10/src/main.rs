use std::fs;
use std::collections::HashMap;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let lines_vector = input.lines().collect::<Vec<_>>();

    let mut map: HashMap<usize, HashMap<usize, (char, bool)>> = HashMap::new();

    let mut s_x = 0;
    let mut s_y = 0;

    for (i, line) in lines_vector.iter().enumerate() {
        let mut line_map: HashMap<usize, (char, bool)> = HashMap::new();
        for (j, char) in line.chars().collect::<Vec<char>>().iter().enumerate() {
            line_map.insert(j, (*char, false));

            if *char == 'S' {
                s_x = j;
                s_y = i;
            }
        }
        map.insert(i, line_map);
    }



    let mut old_x = s_x;
    let mut old_y = s_y;

    map.get_mut(&s_y).unwrap().insert(s_x, ('S', true));

    let mut new_x = 0;
    let mut new_y = 0;

    let mut find_s_vec: Vec<i32> = vec![];

    if s_x != 0 {
        match map[&s_y][&(s_x - 1)].0 {
            '-' | 'L' | 'F' => {
                new_x = s_x - 1;
                new_y = s_y;
                find_s_vec.push(0)
            },
            _ => {},
        }
    }
    if s_x != lines_vector[0].len() - 1 {
        match map[&s_y][&(s_x + 1)].0 {
            '-' | 'J' | '7' => {
                new_x = s_x + 1;
                new_y = s_y;
                find_s_vec.push(1)
            },
            _ => {},
        }
    }
    if s_y != lines_vector.len() - 1 {
        match map[&(s_y + 1)][&s_x].0 {
            '|' | 'J' | 'L' => {
                new_x = s_x;
                new_y = s_y + 1;
                find_s_vec.push(2)
            },
            _ => {},
        }
    }
    if s_y != 0 {
        match map[&(s_y - 1)][&s_x].0 {
            '|' | 'F' | '7' => {
                new_x = s_x;
                new_y = s_y - 1;
                find_s_vec.push(3)
            },
            _ => {},
        }
    }

    println!("{:?}", find_s_vec);

    let s_pipe: char = if find_s_vec[0] == 0 {
        if find_s_vec[1] == 1 {
            '-'
        } else if find_s_vec[1] == 2 {
            '7'
        } else if find_s_vec[1] == 3 {
            'J'
        } else {
            panic!()
        }
    } else if find_s_vec[0] == 1 {
        if find_s_vec[1] == 2 {
            'F'
        } else if find_s_vec[1] == 3 {
            'L'
        } else {
            panic!()
        }
    } else if find_s_vec[0] == 2 {
        '|'
    } else {
        panic!()
    };

    println!("{s_pipe}");

    let mut length_of_pipes = 1;

    loop {
        let pipe = map[&new_y][&new_x].0;
        map.get_mut(&new_y).unwrap().insert(new_x, (pipe, true));

        let temp_x = new_x;
        let temp_y = new_y;

        if pipe == '-' {
            if new_x > old_x {
                new_x += 1;
            } else {
                new_x -= 1;
            }
        } else if pipe == '|' {
            if new_y > old_y {
                new_y += 1;
            } else {
                new_y -= 1;
            }
        } else if pipe == 'L' {
            if new_y > old_y {
                new_x += 1;
            } else {
                new_y -= 1;
            }
        } else if pipe == 'J' {
            if new_y > old_y {
                new_x -= 1;
            } else {
                new_y -= 1;
            }
        } else if pipe == '7' {
            if new_x > old_x {
                new_y += 1;
            } else {
                new_x -= 1;
            }
        } else if pipe == 'F' {
            if new_x < old_x {
                new_y += 1;
            } else {
                new_x += 1;
            }
        } else if pipe == 'S'{
            break
        }

        old_x = temp_x;
        old_y = temp_y;

        length_of_pipes += 1;
    }

    println!("Part1: {}", length_of_pipes/2);

    //part2
    let mut enclosed_count = 0;
    let mut flag;

    for line in map.values() {
        flag = false;
        for x in 0..line.len() {
            let (char, used_pipe) = line[&x];

            let flag_changing_chars = ['|', 'F', '7'];
            if (flag_changing_chars.contains(&char) || (char == 'S' && flag_changing_chars.contains(&s_pipe))) && used_pipe {
                flag = !flag;
            }

            if !used_pipe && flag {
                println!("{:?}, x:{x}", line);
                enclosed_count += 1;
            }
        }
    }
    println!("Enclosed: {enclosed_count} (part2)")
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}